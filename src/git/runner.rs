use crate::Result;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::process::Command;

use super::git_err::GitError;

#[cfg_attr(test, automock)]
pub trait Runner {
	fn run_hook(&self, hook: &str) -> Result<()>;
	fn open_editor(&self, editor: &str, path: &str) -> Result<()>;
}

pub struct CommandRunner {
	shell: String,
}

impl CommandRunner {
	pub fn new() -> Self {
		Self {
			shell: "sh".to_string(),
		}
	}
}

impl Runner for CommandRunner {
	fn run_hook(&self, hook: &str) -> Result<()> {
		if Command::new(self.shell.clone())
			.arg(hook)
			.status()
			.map_err(|_| GitError::Hook)?
			.success()
		{
			return Ok(());
		}
		Err(Box::new(GitError::Hook))
	}

	fn open_editor(&self, editor: &str, path: &str) -> Result<()> {
		Ok(Command::new(editor)
			.arg(path)
			.spawn()
			.map_err(|_| GitError::Editor)
			.map(|_| ())?)
	}
}
