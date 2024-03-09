use std::env;

use super::super::author::{Author, AuthorsProvider};
use super::mapper;
use crate::authors::err::AuthorsError;
use crate::common::conf;
use crate::common::fs::file::File;
use crate::common::fs::wrapper::FileLoader;
use crate::Result;

type OptionalFile = Option<Box<dyn File>>;

pub struct CSVReader {
	src: Box<dyn File>,
}

impl CSVReader {
	pub fn from(file_loader: &impl FileLoader, authors_file: &str) -> Result<Self> {
		let given_file = file_loader.load_if_present(authors_file.to_string());
		match given_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorsError::NotFound(authors_file.to_string()).into()),
		}
	}

	pub fn from_cwd_fallback_home(file_loader: &impl FileLoader) -> Result<Self> {
		let file_path = &conf::authors_file();
		let dir_path = &conf::authors_dir();

		let authors_file = match env::current_dir() {
			Err(_) => Self::xdg_fallback(file_loader, dir_path, file_path),
			Ok(cwd) => file_loader
				.load_if_present(format!("{}/{file_path}", cwd.display()))
				.or_else(|| Self::xdg_fallback(file_loader, dir_path, file_path)),
		};

		match authors_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorsError::NotFound("$PWD or $HOME".to_string()).into()),
		}
	}

	fn xdg_fallback(file_loader: &impl FileLoader, authors_dir: &str, file_path: &str) -> OptionalFile {
		match env::var("XDG_CONFIG_HOME") {
			Err(_) => Self::home_fallback(file_loader, authors_dir, file_path),
			Ok(xdg_config) => file_loader
				.load_if_present(format!("{xdg_config}/{authors_dir}/{file_path}"))
				.or_else(|| Self::home_fallback(file_loader, authors_dir, file_path)),
		}
	}

	fn home_fallback(file_loader: &impl FileLoader, authors_dir: &str, file_path: &str) -> OptionalFile {
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
