pub mod app_service;
pub mod author;
pub mod fs_repo;

pub fn fs_setup_from_file(authors_file: String) -> Result<app_service::AuthorsService<fs_repo::FSRepo>, String> {
	return match fs_repo::FSRepo::from(authors_file) {
		Ok(repo) => Ok(app_service::AuthorsService::new(repo)),
		Err(e) => Err(format!("Couldn't load authors file: {}", e)),
	};
}

pub fn fs_default_setup() -> Result<app_service::AuthorsService<fs_repo::FSRepo>, String> {
	return match fs_repo::FSRepo::default() {
		Ok(repo) => Ok(app_service::AuthorsService::new(repo)),
		Err(e) => Err(format!("Couldn't load authors file: {}", e)),
	};
}
