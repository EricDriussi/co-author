use std::{
	env,
	error::Error,
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
	result::Result,
};

use crate::conf;

use super::{
	author::{Author, AuthorsRepo},
	author_err::AuthorError,
	csv_mapper::CsvMapper,
};

pub struct FSRepo {
	src: PathBuf,
}

impl FSRepo {
	pub fn from_cwd_with_home_fallback() -> Result<Self, Box<dyn Error>> {
		let mut local_file = env::current_dir().unwrap();
		local_file.push(conf::authors_file_name());
		if local_file.is_file() {
			return Ok(Self { src: local_file });
		}

		let default_file = PathBuf::from(conf::authors_file_path());
		if default_file.is_file() {
			return Ok(Self { src: default_file });
		}
		Err(AuthorError::with("No file found!".to_string()))
	}

	pub fn from(authors_file: String) -> Result<Self, Box<dyn Error>> {
		let path = PathBuf::from(authors_file);
		match path.is_file() {
			true => Ok(Self { src: path }),
			false => Err(AuthorError::with(format!(
				"No file at path {:?}",
				path.to_str().unwrap()
			))),
		}
	}

	fn read_lines(&self) -> Vec<String> {
		match File::open(&self.src) {
			Err(_) => Vec::new(),
			Ok(file) => BufReader::new(file).lines().map_while(Result::ok).collect(),
		}
	}
}

impl AuthorsRepo for FSRepo {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		self.read_lines()
			.iter()
			.filter_map(|line| CsvMapper::to_author(line.as_str()))
			.filter(|author| aliases.contains(&author.alias()))
			.collect()
	}

	fn all(&self) -> Vec<Author> {
		self.read_lines()
			.iter()
			.filter_map(|line| CsvMapper::to_author(line.as_str()))
			.collect()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// TODO: do I need this?
	#[test]
	fn should_read_lines() {
		let repo = FSRepo::from(conf::dummy_data()).unwrap();
		let contents = repo.read_lines();

		assert!(!contents.is_empty());
	}
}
