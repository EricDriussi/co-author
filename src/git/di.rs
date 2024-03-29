use super::editor::conf_provider::GitConfProvider;
use super::editor::simple_editor::SimpleEditor;
use super::hook::Hook;
use super::libgit::wrapper::LibGitWrapper;
use super::service::GitService;
use crate::common::file_reader::SimpleReader;
use crate::common::file_writer::SimpleWriter;
use crate::common::runner::CommandRunner;
use crate::Result;

type TextEditor = SimpleEditor<CommandRunner, GitConfProvider>;
type GitHook = Hook<CommandRunner>;
pub type Service = GitService<LibGitWrapper, GitHook, TextEditor, SimpleWriter>;

pub fn init() -> Result<Service> {
	let cwd = std::env::current_dir().map_err(|_| "Not in a valid git repo")?;
	match LibGitWrapper::from(&cwd, &SimpleReader::new()) {
		Ok(wrapper) => GitService::new(
			wrapper,
			Hook::new(CommandRunner::new()),
			SimpleEditor::new(CommandRunner::new(), GitConfProvider::new()),
			SimpleWriter::new(),
		),
		Err(e) => Err(e),
	}
}
