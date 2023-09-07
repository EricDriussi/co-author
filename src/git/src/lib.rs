use std::env;

mod editor;
pub mod git_domain;
pub mod libgit_wrapper;
pub mod service;

pub fn libgit_setup() -> Result<service::GitService<libgit_wrapper::LibGitWrapper>, String> {
	let serv = match libgit_wrapper::LibGitWrapper::from(env::current_dir().unwrap()) {
		Ok(repo) => Ok(service::GitService::new(repo)),
		Err(e) => Err(e),
	};
	return serv;
}
