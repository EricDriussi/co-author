use super::super::author::{Author, AuthorsProvider};
use super::super::author_err::AuthorError;
use super::csv_mapper;
use std::{error::Error, result::Result};

use crate::conf;
use crate::fs::{FileLoader, Readable};

pub struct CSVReader {
	src: Box<dyn Readable>,
}

impl CSVReader {
	pub fn from_cwd_fallback_home(file_loader: &impl FileLoader) -> Result<Self, Box<dyn Error>> {
		let file_in_cwd = file_loader.load_file(conf::authors_csv_file());
		if let Some(file) = file_in_cwd {
			return Ok(Self { src: file });
		}
		let file_in_home = file_loader.load_file(conf::authors_csv_path());
		match file_in_home {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorError::with("No file found in cwd or home".to_string())),
		}
	}

	pub fn from(file_loader: &impl FileLoader, authors_file: &str) -> Result<Self, Box<dyn Error>> {
		let given_file = file_loader.load_file(authors_file.to_string());
		match given_file {
			Some(file) => Ok(Self { src: file }),
			None => Err(AuthorError::with(format!(
				"No file at path: {:?}",
				authors_file.to_string()
			))),
		}
	}
}

impl AuthorsProvider for CSVReader {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		self.src
			.non_empty_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.src
			.non_empty_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.collect()
	}
}
