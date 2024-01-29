use std::error::Error;

use crate::fs::wrapper::FileLoader;

use super::{
	commit_body::{CommitBody, GitWrapper},
	conf_provider::ConfProvider,
	editor::TextEditor,
	hook_runner::HookRunner,
	runner::Runner,
};

pub struct GitService<T: GitWrapper, R: Runner, F: FileLoader, C: ConfProvider> {
	git_wrapper: T,
	hook_runner: HookRunner,
	text_editor: TextEditor<R, F, C>,
}

impl<T: GitWrapper, R: Runner, F: FileLoader, C: ConfProvider> GitService<T, R, F, C> {
	pub fn new(repo: T, runner: R, file_loader: F, conf_provider: C) -> Self {
		Self {
			hook_runner: HookRunner::new(),
			git_wrapper: repo,
			text_editor: TextEditor::new(runner, file_loader, conf_provider),
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
		self.text_editor.open_editmsg()?;
		self.hook_runner.commit_msg()?;
		self.git_wrapper.commit()
	}

	pub fn commit_with_pre_populated_editor(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new(message, authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		self.text_editor.open_editmsg()?;
		self.hook_runner.commit_msg()?;
		self.git_wrapper.commit()
	}
}
