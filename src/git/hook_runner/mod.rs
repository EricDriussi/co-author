use std::{error::Error, path::PathBuf, process::Command};

mod hook_err;
use co_author::conf;

use self::hook_err::HookError;

pub struct HookRunner {
	path: String,
}

impl HookRunner {
	pub fn new() -> Self {
		Self {
			path: conf::hooks_path(),
		}
	}

	pub fn pre_commit(&self) -> Result<(), Box<dyn Error>> {
		let hook = PathBuf::from(format!("{}/pre-commit", self.path));
		if !hook.exists() {
			return Ok(());
		}

		return if Command::new(hook).status().is_ok_and(|s| s.success()) {
			Ok(())
		} else {
			Err(HookError::with("Pre-commit"))
		};
	}

	pub fn commit_msg(&self) -> Result<(), Box<dyn Error>> {
		let hook = PathBuf::from(format!("{}/commit-msg", self.path));
		if !hook.exists() {
			return Ok(());
		}

		return if Command::new(hook)
			.arg(conf::editmsg())
			.status()
			.is_ok_and(|s| s.success())
		{
			Ok(())
		} else {
			Err(HookError::with("Commit-msg"))
		};
	}
}

#[cfg(test)]
mod test;
