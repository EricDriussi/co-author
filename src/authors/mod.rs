use std::error::Error;

use crate::{authors::csv_provider::provider::CSVReader, fs_wrapper};
use author_err::AuthorError;
use service::AuthorsService;

pub mod author;
mod author_err;
mod csv_provider;
pub mod service;

pub fn from_file(authors_file: &str) -> Result<AuthorsService<CSVReader>, Box<dyn Error>> {
	let fs_wrapper = fs_wrapper::SimpleFsWrapper::new();
	match CSVReader::from(&fs_wrapper, authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

pub fn default() -> Result<AuthorsService<CSVReader>, Box<dyn Error>> {
	let fs_wrapper = fs_wrapper::SimpleFsWrapper::new();
	match CSVReader::from_cwd_with_home_fallback(&fs_wrapper) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

#[cfg(test)]
mod test;
