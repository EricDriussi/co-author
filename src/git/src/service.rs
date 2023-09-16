use std::error::Error;

use crate::{
	editor,
	git::{CommitBody, GitWrapper},
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

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(CommitBody::new(message, authors))?;
		self.hook_runner.commit_msg()?;
		return self.git_wrapper.commit();
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), Box<dyn Error>> {
		self.hook_runner.pre_commit()?;
		self.git_wrapper.write_to_editmsg(CommitBody::new("", authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		editor::open();
		self.hook_runner.commit_msg()?;
		return self.git_wrapper.commit();
	}
}
