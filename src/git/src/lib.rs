use std::env;

pub mod app_service;
mod editor;
pub mod git;
pub mod libgit_repo;

pub fn libgit_setup() -> Result<app_service::GitService<libgit_repo::LibGitRepo>, String> {
	let repo = libgit_repo::LibGitRepo::new(env::current_dir().unwrap());
	let serv = match repo.open_if_valid() {
		Some(repo) => Ok(app_service::GitService::new(repo)),
		None => {
			return Err("Not a valid git repository".to_string());
		}
	};
	return serv;
}
