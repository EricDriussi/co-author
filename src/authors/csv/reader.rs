use super::super::author::{Author, AuthorsProvider};
use super::mapper;
use crate::authors::err::AuthorsError;
use crate::common::conf;
use crate::common::fs::file::File;
use crate::common::fs::wrapper::{FileLoader, OptionalFile};
use crate::Result;
use std::env;

pub enum LoadMode<'a> {
	FromCwd {
		file_loader: &'a dyn FileLoader,
	},
	FromPath {
		file_loader: &'a dyn FileLoader,
		path: &'a str,
	},
}

pub struct CSVReader {
	src: Box<dyn File>,
}

impl CSVReader {
	pub fn load(load_mode: &LoadMode) -> Result<Self> {
		match load_mode {
			LoadMode::FromPath { file_loader, path } => CSVReader::from_file(file_loader.to_owned(), path),
			LoadMode::FromCwd { file_loader } => CSVReader::from_cwd_fallback_home(file_loader.to_owned()),
		}
	}

	fn from_file(file_loader: &dyn FileLoader, authors_file: &str) -> Result<Self> {
		let given_file = file_loader.load_if_present(authors_file.to_string());
		match given_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorsError::NotFound(authors_file.to_string()).into()),
		}
	}

	fn from_cwd_fallback_home(file_loader: &dyn FileLoader) -> Result<Self> {
		let file_path = &conf::authors_file();
		let dir_path = &conf::authors_dir();

		let authors_file = env::current_dir()
			.ok()
			.and_then(|cwd| file_loader.load_if_present(format!("{}/{file_path}", cwd.display())))
			.or_else(|| Self::xdg_or_home_fallback(file_loader, dir_path, file_path));

		authors_file
			.map(|file| Self { src: file })
			.ok_or(AuthorsError::NotFound("$PWD or $HOME".to_string()).into())
	}

	fn xdg_or_home_fallback(file_loader: &dyn FileLoader, authors_dir: &str, file_path: &str) -> OptionalFile {
		env::var("XDG_CONFIG_HOME")
			.ok()
			.and_then(|xdg_config| file_loader.load_if_present(format!("{xdg_config}/{authors_dir}/{file_path}")))
			.or_else(|| Self::home_fallback(file_loader, authors_dir, file_path))
	}

	fn home_fallback(file_loader: &dyn FileLoader, authors_dir: &str, file_path: &str) -> OptionalFile {
		let home = env::var("HOME").ok()?;
		file_loader
			.load_if_present(format!("{home}/.config/{authors_dir}/{file_path}"))
			.or_else(|| file_loader.load_if_present(format!("{home}/.{authors_dir}/{file_path}")))
			.or_else(|| file_loader.load_if_present(format!("{home}/{file_path}")))
	}
}

impl AuthorsProvider for CSVReader {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		self.src
			.non_empty_lines()
			.iter()
			.filter_map(|line| mapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.src
			.non_empty_lines()
			.iter()
			.filter_map(|line| mapper::to_author(line.as_str()))
			.collect()
	}
}
