use std::error::Error;

use author_err::AuthorError;
use fs_repo::FSRepo;
use service::AuthorsService;

pub mod author;
mod author_err;
pub mod fs_repo;
pub mod service;

pub fn fs_setup_from_file(authors_file: String) -> Result<AuthorsService<FSRepo>, Box<dyn Error>> {
	match FSRepo::from(authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {}", e))),
	}
}

pub fn new_fs_default_setup() -> Result<AuthorsService<FSRepo>, Box<dyn Error>> {
	match FSRepo::new_default() {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::with(format!("Couldn't load file: {}", e))),
	}
}

#[cfg(test)]
mod test;
