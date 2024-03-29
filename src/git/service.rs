use super::commit_message::{CommitMessage, GitWrapper};
use super::commit_mode::CommitMode;
use super::editor::simple_editor::Editor;
use super::err::GitError;
use super::hook::HookRunner;
use crate::common::conf;
use crate::common::fs::file::File;
use crate::common::fs::wrapper::FileLoader;
use crate::Result;

pub struct GitService<W: GitWrapper, H: HookRunner, E: Editor> {
	git_wrapper: W,
	hook_runner: H,
	editmsg: Box<dyn File>,
	editmsg_editor: E,
}

impl<W: GitWrapper, H: HookRunner, E: Editor> GitService<W, H, E> {
	pub fn new(git_wrapper: W, runner: H, file_loader: &dyn FileLoader, editmsg_editor: E) -> Result<Self> {
		let editmsg = file_loader.load_or_create(conf::editmsg()).ok_or(GitError::Editmsg)?;
		Ok(Self {
			git_wrapper,
			hook_runner: runner,
			editmsg,
			editmsg_editor,
		})
	}

	pub fn last_commit_message(&self) -> String {
		self.git_wrapper
			.prev_commit_msg()
			.unwrap_or_default()
			.subject()
			.to_string()
	}

	pub fn commit(&mut self, commit_mode: CommitMode) -> Result<()> {
		match commit_mode {
			CommitMode::WithoutEditor { message, authors } => {
				self.pre(&CommitMessage::new(message, authors))?;
				self.run_commit()
			}
			CommitMode::WithEditor { message, authors } => {
				self.pre(&CommitMessage::new(message.unwrap_or_default(), authors))?;
				self.editor()?;
				self.run_commit()
			}
		}
	}

	fn pre(&mut self, body: &CommitMessage) -> Result<()> {
		self.hook_runner.run_pre_commit()?;
		self.editmsg.write(body.to_string())
	}

	fn editor(&mut self) -> Result<()> {
		let status = self.git_wrapper.formatted_status()?;
		self.editmsg.write(status)?;
		self.editmsg_editor.open(self.editmsg.as_ref())
	}

	fn run_commit(&self) -> Result<()> {
		self.hook_runner.run_commit_msg()?;
		self.git_wrapper.commit()
	}
}
