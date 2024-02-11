use std::error::Error;

use super::{
	commit_body::{CommitBody, GitWrapper},
	editor::EditmsgEditor,
	hook_runner::HookRunner,
};

pub struct GitService<W: GitWrapper, E: EditmsgEditor> {
	git_wrapper: W,
	hook_runner: HookRunner,
	editmsg_editor: E,
}

impl<W: GitWrapper, E: EditmsgEditor> GitService<W, E> {
	pub fn new(git_wrapper: W, editmsg_editor: E, hook_runner: HookRunner) -> Self {
		Self {
			git_wrapper,
			hook_runner,
			editmsg_editor,
		}
	}

	pub fn last_commit_message(&self) -> String {
		self.git_wrapper.prev_commit_msg().unwrap_or_default()
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new(message, authors))?;
		self.hook_runner.commit_msg()?;
		self.git_wrapper.commit()
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new("", authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		self.editmsg_editor.open()?;
		self.hook_runner.commit_msg()?;
		self.git_wrapper.commit()
	}

	pub fn commit_with_pre_populated_editor(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new(message, authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		self.editmsg_editor.open()?;
		self.hook_runner.commit_msg()?;
		self.git_wrapper.commit()
	}
}
