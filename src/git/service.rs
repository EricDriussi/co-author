use std::error::Error;

use super::{
	commit_body::{CommitBody, GitWrapper},
	editor,
	hook_runner::HookRunner,
};

pub struct GitService<T: GitWrapper> {
	git_wrapper: T,
	hook_runner: HookRunner,
}

impl<T: GitWrapper> GitService<T> {
	pub fn new(repo: T) -> GitService<T> {
		GitService {
			hook_runner: HookRunner::new(),
			git_wrapper: repo,
		}
	}

	pub fn last_commit_message(&self) -> String {
		self.git_wrapper
			.prev_commit_msg()
			.unwrap_or_default()
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner
			.pre_commit()?;
		self.git_wrapper
			.write_to_editmsg(CommitBody::new(message, authors))?;
		self.hook_runner
			.commit_msg()?;
		self.git_wrapper
			.commit()
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner
			.pre_commit()?;
		self.git_wrapper
			.write_to_editmsg(CommitBody::new("", authors))?;
		self.git_wrapper
			.add_status_to_editmsg()?;
		editor::open();
		self.hook_runner
			.commit_msg()?;
		self.git_wrapper
			.commit()
	}

	pub fn commit_with_pre_populated_editor(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner
			.pre_commit()?;
		self.git_wrapper
			.write_to_editmsg(CommitBody::new(message, authors))?;
		self.git_wrapper
			.add_status_to_editmsg()?;
		editor::open();
		self.hook_runner
			.commit_msg()?;
		self.git_wrapper
			.commit()
	}
}
