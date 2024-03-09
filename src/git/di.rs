use super::conf_provider::GitConfProvider;
use super::editor::Editor;
use super::hook::Hook;
use super::libgit_wrapper::LibGitWrapper;
use super::service::GitService;
use crate::common::fs::wrapper::FsWrapper;
use crate::common::runner::CommandRunner;
use crate::Result;
use std::path::PathBuf;

type TextEditor = Editor<CommandRunner, FsWrapper, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
type Service = GitService<LibGitWrapper, GitHook, TextEditor>;

pub fn init_git_module(dir: &PathBuf) -> Result<Service> {
	match LibGitWrapper::from(dir, &FsWrapper::new()) {
		Ok(wrapper) => GitService::new(
			wrapper,
			Hook::new(CommandRunner::new()),
			&FsWrapper::new(),
			Editor::new(CommandRunner::new(), FsWrapper::new(), GitConfProvider::new()),
		),
		Err(e) => Err(e),
	}
}
