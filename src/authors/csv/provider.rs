use super::super::author::{Author, AuthorsProvider};
use super::mapper;

use crate::authors::authors_err::AuthorsError;
use crate::common::conf;
use crate::common::fs::file::File;
use crate::common::fs::wrapper::FileLoader;
use crate::Result;

pub struct CSVReader {
	src: Box<dyn File>,
}

impl CSVReader {
	pub fn from_cwd_fallback_home(file_loader: &impl FileLoader) -> Result<Self> {
		let file_in_home = file_loader.load_file_with_fallback(conf::authors_file());
		match file_in_home {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorsError::NotFound("$PWD or $HOME".to_string()).into()),
		}
	}

	pub fn from(file_loader: &impl FileLoader, authors_file: &str) -> Result<Self> {
		let given_file = file_loader.load_if_present(authors_file.to_string());
		match given_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorsError::NotFound(authors_file.to_string()).into()),
		}
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
