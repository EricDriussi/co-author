use crate::Result;

use super::hook::HookRunner;
use super::{
	commit_body::{CommitBody, GitWrapper},
	editor::EditmsgEditor,
};

pub struct GitService<W: GitWrapper, H: HookRunner, E: EditmsgEditor> {
	git_wrapper: W,
	hook_runner: H,
	editmsg_editor: E,
}

pub enum CommitMode<'a> {
	WithEditor {
		message: Option<&'a str>,
		authors: Vec<String>,
	},
	WithoutEditor {
		message: &'a str,
		authors: Vec<String>,
	},
}

impl<W: GitWrapper, H: HookRunner, E: EditmsgEditor> GitService<W, H, E> {
	pub fn new(git_wrapper: W, runner: H, editmsg_editor: E) -> Self {
		Self {
			git_wrapper,
			hook_runner: runner,
			editmsg_editor,
		}
	}

	pub fn last_commit_message(&self) -> String {
		self.git_wrapper.prev_commit_msg().unwrap_or_default()
	}

	pub fn commit(&self, commit_mode: CommitMode) -> Result<()> {
		match commit_mode {
			CommitMode::WithoutEditor { message, authors } => {
				self.pre(&CommitBody::new(message, authors))?;
				self.run_commit()
			}
			CommitMode::WithEditor { message, authors } => {
				self.pre(&CommitBody::new(message.unwrap_or_default(), authors))?;
				self.editor()?;
				self.run_commit()
			}
		}
	}

	fn pre(&self, body: &CommitBody) -> Result<()> {
		self.hook_runner.run_pre_commit()?;
		self.git_wrapper.write_to_editmsg(body)
	}

	fn editor(&self) -> Result<()> {
		self.git_wrapper.add_status_to_editmsg()?;
		self.editmsg_editor.open()
	}

	fn run_commit(&self) -> Result<()> {
		self.hook_runner.run_commit_msg()?;
		self.git_wrapper.commit()
	}
}
