use super::editor::conf_provider::GitConfProvider;
use super::editor::simple_editor::SimpleEditor;
use super::hook::Hook;
use super::libgit::wrapper::LibGitWrapper;
use super::service::GitService;
use crate::common::fs::wrapper::FsWrapper;
use crate::common::runner::CommandRunner;
use crate::Result;

type TextEditor = SimpleEditor<CommandRunner, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
type Service = GitService<LibGitWrapper, GitHook, TextEditor>;

pub fn init(dir: &str) -> Result<Service> {
	match LibGitWrapper::from(dir, &FsWrapper::new()) {
		Ok(wrapper) => GitService::new(
			wrapper,
			Hook::new(CommandRunner::new()),
			&FsWrapper::new(),
			SimpleEditor::new(CommandRunner::new(), GitConfProvider::new()),
		),
		Err(e) => Err(e),
	}
}
