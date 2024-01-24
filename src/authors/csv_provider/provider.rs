use super::super::author::{Author, AuthorsProvider};
use super::super::author_err::AuthorError;
use super::csv_mapper;
use std::{error::Error, result::Result};

use crate::conf;
use crate::fs_wrapper::{File, FsWrapper};

pub struct CSVReader {
	src: Box<dyn File>,
}

impl CSVReader {
	pub fn from_cwd_with_home_fallback(fs_wrapper: &impl FsWrapper) -> Result<Self, Box<dyn Error>> {
		let file_in_cwd = fs_wrapper.file_in_cwd(conf::authors_file_name());
		if let Some(file) = file_in_cwd {
			return Ok(Self { src: file });
		}
		let file_in_home = fs_wrapper.file_in_abs_path(conf::authors_file_path());
		match file_in_home {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorError::with("No file found!".to_string())),
		}
	}

	pub fn from(fs_wrapper: &impl FsWrapper, authors_file: &str) -> Result<Self, Box<dyn Error>> {
		let given_file = fs_wrapper.file_in_abs_path(authors_file.to_string());

		match given_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorError::with(format!(
				"No file at path {:?}",
				authors_file.to_string()
			))),
		}
	}
}

impl AuthorsProvider for CSVReader {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		self.src
			.read_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.src
			.read_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.collect()
	}
}
