use super::super::author::{Author, AuthorsProvider};
use super::mapper;
use crate::authors::err::AuthorsError;
use crate::common::conf;
use crate::common::err::SystemError;
use crate::common::fs::file_reader::{Lines, Reader};
use crate::Result;
use std::env;
use std::path::{Path, PathBuf};

pub enum LoadMode<'a> {
	FromCwd { file_reader: &'a dyn Reader },
	FromPath { file_reader: &'a dyn Reader, path: PathBuf },
}

pub struct CSVReader {
	lines: Vec<String>,
}

impl CSVReader {
	pub fn load(load_mode: &LoadMode) -> Result<Self> {
		match load_mode {
			LoadMode::FromPath { file_reader, path } => CSVReader::from_file(file_reader.to_owned(), path),
			LoadMode::FromCwd { file_reader } => CSVReader::from_cwd_fallback_home(file_reader.to_owned()),
		}
	}

	fn from_file(file_reader: &dyn Reader, authors_file: &Path) -> Result<Self> {
		file_reader.read_non_empty_lines(authors_file).map_or(
			Err(AuthorsError::NotFound(authors_file.to_string_lossy().to_string()).into()),
			|lines| Ok(Self { lines }),
		)
	}

	fn from_cwd_fallback_home(file_reader: &dyn Reader) -> Result<Self> {
		let file_path = &conf::authors_file();
		let dir_path = &conf::authors_dir();
		let cwd = env::current_dir().map_err(|_| SystemError::EnvVar("CWD".to_string()))?;

		file_reader
			.read_non_empty_lines(&cwd.join(file_path))
			.or_else(|_| Self::xdg_or_home_fallback(file_reader, dir_path, file_path))
			.or_else(|_| Self::home_fallback(file_reader, dir_path, file_path))
			.map_or(
				Err(AuthorsError::NotFound("$PWD or $HOME".to_string()).into()),
				|lines| Ok(Self { lines }),
			)
	}

	fn xdg_or_home_fallback(file_reader: &dyn Reader, authors_dir: &str, file_path: &str) -> Result<Lines> {
		let home = PathBuf::from(env::var("XDG_CONFIG_HOME")?);
		file_reader.read_non_empty_lines(&home.join(authors_dir).join(file_path))
	}

	fn home_fallback(file_reader: &dyn Reader, authors_dir: &str, file_path: &str) -> Result<Lines> {
		let home = env::var("HOME")?;
		file_reader
			.read_non_empty_lines(&PathBuf::from(format!("{home}/.config/{authors_dir}/{file_path}")))
			.or_else(|_| file_reader.read_non_empty_lines(&PathBuf::from(format!("{home}/.{authors_dir}/{file_path}"))))
			.or_else(|_| file_reader.read_non_empty_lines(&PathBuf::from(format!("{home}/{file_path}"))))
	}
}

impl AuthorsProvider for CSVReader {
	fn find(&self, aliases: &[String]) -> Vec<Author> {
		self.lines
			.iter()
			.filter_map(|line| mapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.lines
			.iter()
			.filter_map(|line| mapper::to_author(line.as_str()))
			.collect()
	}
}
