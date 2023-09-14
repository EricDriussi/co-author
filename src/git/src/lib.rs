use std::env;

use libgit_wrapper::LibGitWrapper;
use service::GitService;

mod editor;
pub mod git_domain;
mod git_err;
mod hook_runner;
pub mod libgit_wrapper;
pub mod service;

pub fn libgit_setup() -> Result<GitService<LibGitWrapper>, String> {
	let serv = match LibGitWrapper::from(env::current_dir().unwrap()) {
		Ok(repo) => Ok(GitService::new(repo)),
		Err(e) => Err(e),
	};
	return serv;
}
