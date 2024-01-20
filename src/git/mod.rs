use std::env;

use libgit_wrapper::LibGitWrapper;
use service::GitService;

pub mod commit_body;
mod editor;
mod hook_runner;
pub mod libgit_wrapper;
pub mod service;

pub fn libgit_setup() -> Result<GitService<LibGitWrapper>, String> {
	match LibGitWrapper::from(&env::current_dir().unwrap()) {
		Ok(repo) => Ok(GitService::new(repo)),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod test;
