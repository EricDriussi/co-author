use super::core::conf_provider::GitConfProvider;
use super::core::editor::file_editor::FileEditor;
use super::core::hook::Hook;
use super::core::libgit::wrapper::LibGitWrapper;
use super::core::service::GitService;
use crate::common::env;
use crate::common::fs::file_reader::FileReader;
use crate::common::fs::file_writer::FileWriter;
use crate::common::runner::CommandRunner;
use crate::Result;

type Editor = FileEditor<CommandRunner, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
pub type Service = GitService<LibGitWrapper<FileReader>, GitHook, Editor, FileWriter>;

pub fn init() -> Result<Service> {
	let cwd = env::cwd()?;
	Ok(GitService::new(
		LibGitWrapper::from(&cwd, FileReader)?,
		Hook::new(CommandRunner),
		FileEditor::new(CommandRunner, GitConfProvider),
		FileWriter,
	))
}
