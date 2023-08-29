use std::env;

mod editor_handler;
pub mod git_domain;
pub mod libgit_wrapper;
pub mod service;

pub fn libgit_setup() -> Result<service::GitService<libgit_wrapper::LibGitWrapper>, String> {
	let repo = libgit_wrapper::LibGitWrapper::new(env::current_dir().unwrap());
	let serv = match repo.open_if_valid() {
		Some(repo) => Ok(service::GitService::new(repo)),
		None => {
			return Err("Not a valid git repository".to_string());
		}
	};
	return serv;
}
