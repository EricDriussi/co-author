use std::env;

use libgit_wrapper::LibGitWrapper;
use service::GitService;

use crate::fs::wrapper::FsWrapper;

use self::{conf_provider::GitConfProvider, runner::CommandRunner};

pub mod commit_body;
mod conf_provider;
mod editor;
mod git_err;
mod hook_runner;
pub mod libgit_wrapper;
mod runner;
pub mod service;

pub fn libgit_setup() -> Result<GitService<LibGitWrapper, CommandRunner, FsWrapper, GitConfProvider>, String> {
	let cwd = env::current_dir().map_err(|_| "Could not get current directory".to_string())?;
	match LibGitWrapper::from(&cwd) {
		Ok(repo) => Ok(GitService::new(
			repo,
			CommandRunner::new(),
			FsWrapper::new(),
			GitConfProvider::new(),
		)),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod test;
