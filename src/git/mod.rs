use self::{conf_provider::GitConfProvider, editor::Editor, hook::Hook};
use crate::common::fs::wrapper::FsWrapper;
use crate::common::runner::CommandRunner;
use crate::Result;
use libgit_wrapper::LibGitWrapper;
use service::GitService;
use std::env;
pub mod commit_message;
mod conf_provider;
mod editor;
mod git_err;
mod hook;
pub mod libgit_wrapper;
pub mod service;

type TextEditor = Editor<CommandRunner, FsWrapper, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
type Service = GitService<LibGitWrapper, GitHook, TextEditor>;

pub fn init_git_dependency_tree() -> Result<Service> {
	let cwd = env::current_dir().map_err(|_| "Could not get current directory".to_string())?;
	match LibGitWrapper::from(&cwd, &FsWrapper::new()) {
		Ok(wrapper) => GitService::new(
			wrapper,
			Hook::new(CommandRunner::new()),
			&FsWrapper::new(),
			Editor::new(CommandRunner::new(), FsWrapper::new(), GitConfProvider::new()),
		),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod test;
