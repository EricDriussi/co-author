use super::core::conf_provider::GitConfProvider;
use super::core::editor::file_editor::FileEditor;
use super::core::hook::Hook;
use super::core::libgit::wrapper::LibGitWrapper;
use super::service::GitService;
use crate::common::env;
use crate::common::fs::file_reader::SimpleReader;
use crate::common::fs::file_writer::SimpleWriter;
use crate::common::runner::CommandRunner;
use crate::Result;

type Editor = FileEditor<CommandRunner, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
pub type Service = GitService<LibGitWrapper<SimpleReader>, GitHook, Editor, SimpleWriter>;

pub fn init() -> Result<Service> {
	let cwd = env::cwd()?;

	let wrapper = LibGitWrapper::from(&cwd, SimpleReader::new())?;
	Ok(GitService::new(
		wrapper,
		Hook::new(CommandRunner::new()),
		FileEditor::new(CommandRunner::new(), GitConfProvider::new()),
		SimpleWriter::new(),
	))
}
