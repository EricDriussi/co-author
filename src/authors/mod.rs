use std::error::Error;

use author_err::AuthorError;
use service::AuthorsService;
use crate::authors::csv_provider::provider::CSVProvider;

pub mod author;
mod author_err;
mod csv_provider;
pub mod service;

pub fn from_file(authors_file: String) -> Result<AuthorsService<CSVProvider>, Box<dyn Error>> {
	match CSVProvider::from(authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

pub fn default() -> Result<AuthorsService<CSVProvider>, Box<dyn Error>> {
	match CSVProvider::from_cwd_with_home_fallback() {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

#[cfg(test)]
mod test;
