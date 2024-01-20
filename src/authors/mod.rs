use std::error::Error;

use author_err::AuthorError;
use fs_provider::FSProvider;
use service::AuthorsService;

pub mod author;
mod author_err;
mod csv_mapper;
pub mod fs_provider;
pub mod service;

pub fn from_file(authors_file: String) -> Result<AuthorsService<FSProvider>, Box<dyn Error>> {
	match FSProvider::from(authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

pub fn default() -> Result<AuthorsService<FSProvider>, Box<dyn Error>> {
	match FSProvider::from_cwd_with_home_fallback() {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {e}"))),
	}
}

#[cfg(test)]
mod test;
