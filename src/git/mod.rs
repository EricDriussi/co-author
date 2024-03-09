use self::{conf_provider::GitConfProvider, editor::Editor, hook::Hook};
use crate::common::fs::wrapper::FsWrapper;
use crate::common::runner::CommandRunner;
use crate::Result;
use libgit_wrapper::LibGitWrapper;
use service::GitService;
use std::env;

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

mod commit_message;
pub mod commit_mode;
mod conf_provider;
mod editor;
mod err;
mod hook;
mod libgit_wrapper;
mod service;

#[cfg(test)]
mod test {
	mod editor_should;
	mod git_should;
	mod hook_should;
	mod libgit_wrapper_should;
	mod service {
		mod commit_with_editor_should;
		mod commit_without_editor_should;
		mod service_should;

		pub mod util {
			pub mod mock_file;
			pub mod mock_helpers;
		}
	}
}
