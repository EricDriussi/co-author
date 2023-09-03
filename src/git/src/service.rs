use std::{path::PathBuf, process::Command};

use crate::{
	editor,
	git_domain::{CommitBody, GitWrapper},
};

pub struct GitService<T: GitWrapper> {
	git_wrapper: T,
}

impl<T: GitWrapper> GitService<T> {
	pub fn new(repo: T) -> GitService<T> {
		GitService { git_wrapper: repo }
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), String> {
		run_pre_commit_hook(self.git_wrapper.hooks_path())?;
		self.git_wrapper.write_to_editmsg(CommitBody::new(message, authors))?;
		// TODO. commit-msg hook (pass editmsg path as param)
		return self.git_wrapper.commit();
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), String> {
		run_pre_commit_hook(self.git_wrapper.hooks_path())?;
		self.git_wrapper.write_to_editmsg(CommitBody::new("", authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		let editmsg_path = self.git_wrapper.editmsg_path();
		editor::open(editmsg_path);
		// TODO. commit-msg hook (pass editmsg path as param)
		return self.git_wrapper.commit();
	}
}

fn run_pre_commit_hook(mut hooks_path: PathBuf) -> Result<(), String> {
	hooks_path.push("pre-commit");
	match hooks_path.exists() {
		true => {
			let status = Command::new(&hooks_path).status();
			let succeeded = status.is_ok() && status.unwrap().success();

			return match succeeded {
				true => Ok(()),
				false => Err("Pre-commit hook failed, aborting".to_string()),
			};
		}
		false => return Ok(()),
	}
}
