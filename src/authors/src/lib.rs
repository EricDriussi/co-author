pub mod app_service;
pub mod author;
pub mod fs_repo;

pub fn fs_setup(authors_file: Option<String>) -> Result<app_service::AuthorsService<fs_repo::FSRepo>, String> {
	return match fs_repo::FSRepo::from(authors_file) {
		Ok(repo) => Ok(app_service::AuthorsService::new(repo)),
		Err(e) => Err(format!("Couldn't load authors file: {}", e)),
	};
}
