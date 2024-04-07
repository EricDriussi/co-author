use super::commit_mode::CommitMode;
use super::core::commit_message::{CommitMessage, GitWrapper};
use super::core::editor::file_editor::Editor;
use super::core::hook::HookRunner;
use crate::common::conf;
use crate::common::fs::file_writer::Writer;
use crate::Result;
use std::path::PathBuf;

pub struct GitService<G: GitWrapper, H: HookRunner, E: Editor, W: Writer> {
	git_wrapper: G,
	hook_runner: H,
	editmsg_editor: E,
	file_writer: W,
	editmsg_path: String,
}

impl<G: GitWrapper, H: HookRunner, E: Editor, W: Writer> GitService<G, H, E, W> {
	pub fn new(git_wrapper: G, runner: H, editmsg_editor: E, file_writer: W) -> Self {
		let editmsg_path = conf::editmsg();
		Self {
			git_wrapper,
			hook_runner: runner,
			editmsg_editor,
			file_writer,
			editmsg_path,
		}
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
		self.file_writer
			.overwrite(&PathBuf::from(&self.editmsg_path), &body.to_string())
	}

	fn editor(&mut self) -> Result<()> {
		let status = self.git_wrapper.formatted_status()?;
		self.file_writer.append(&PathBuf::from(&self.editmsg_path), &status)?;
		self.editmsg_editor.open(&self.editmsg_path)
	}

	fn run_commit(&self) -> Result<()> {
		self.hook_runner.run_commit_msg()?;
		self.git_wrapper.commit()
	}
}
