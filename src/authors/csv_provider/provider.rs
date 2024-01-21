use super::super::author::{Author, AuthorsProvider};
use super::super::author_err::AuthorError;
use super::csv_mapper;
use std::{
	env,
	error::Error,
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
	result::Result,
};

use crate::conf;

pub struct CSVProvider {
	src: PathBuf,
}

impl CSVProvider {
	pub fn from_cwd_with_home_fallback() -> Result<Self, Box<dyn Error>> {
		let file_in_cwd = env::current_dir()?.join(conf::authors_file_name());
		if file_in_cwd.is_file() {
			return Ok(Self { src: file_in_cwd });
		}
		let file_in_home = PathBuf::from(conf::authors_file_path());
		if file_in_home.is_file() {
			return Ok(Self { src: file_in_home });
		}
		Err(AuthorError::with("No file found!".to_string()))
	}

	pub fn from(authors_file: String) -> Result<Self, Box<dyn Error>> {
		let given_file = PathBuf::from(authors_file);
		if given_file.is_file() {
			return Ok(Self { src: given_file });
		}
		Err(AuthorError::with(format!(
			"No file at path {:?}",
			given_file.to_str().ok_or("?")
		)))
	}

	fn read_lines(&self) -> Vec<String> {
		match File::open(&self.src) {
			Err(_) => Vec::new(),
			Ok(file) => BufReader::new(file).lines().map_while(Result::ok).collect(),
		}
	}
}

impl AuthorsProvider for CSVProvider {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		self.read_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.read_lines()
			.iter()
			.filter_map(|line| csv_mapper::to_author(line.as_str()))
			.collect()
	}
}
