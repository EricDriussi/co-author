use std::env;

use libgit_wrapper::LibGitWrapper;
use service::GitService;

use crate::fs::wrapper::FsWrapper;

use self::{conf_provider::GitConfProvider, editor::Editor, hook_runner::HookRunner, runner::CommandRunner};

pub mod commit_body;
mod conf_provider;
mod editor;
mod git_err;
mod hook_runner;
pub mod libgit_wrapper;
mod runner;
pub mod service;

pub fn init_git_dependency_tree(
) -> Result<GitService<LibGitWrapper, Editor<CommandRunner, FsWrapper, GitConfProvider>>, String> {
	let cwd = env::current_dir().map_err(|_| "Could not get current directory".to_string())?;
	match LibGitWrapper::from(&cwd) {
		Ok(wrapper) => Ok(GitService::new(
			wrapper,
			Editor::new(CommandRunner::new(), FsWrapper::new(), GitConfProvider::new()),
			HookRunner::new(),
		)),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod test;
