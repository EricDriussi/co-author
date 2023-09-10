use std::error::Error;

use app_service::AuthorsService;
use author_err::AuthorError;
use fs_repo::FSRepo;

pub mod app_service;
pub mod author;
mod author_err;
pub mod fs_repo;

pub fn fs_setup_from_file(authors_file: String) -> Result<AuthorsService<FSRepo>, Box<dyn Error>> {
	return match FSRepo::from(authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::new(format!("Couldn't load file: {}", e))),
	};
}

pub fn fs_default_setup(default_authors_file: String) -> Result<AuthorsService<FSRepo>, Box<dyn Error>> {
	return match FSRepo::default(default_authors_file) {
		Ok(repo) => Ok(AuthorsService::new(repo)),
		Err(e) => Err(AuthorError::new(format!("Couldn't load file: {}", e))),
	};
}
