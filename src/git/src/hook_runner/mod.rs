use std::{error::Error, path::PathBuf, process::Command};

use crate::git_err::HookError;

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
		let p = PathBuf::from(format!("{}/pre-commit", self.path));
		return match p.exists() {
			true => {
				let status = Command::new(p).status();
				let succeeded = status.is_ok() && status.unwrap().success();

				match succeeded {
					true => Ok(()),
					false => Err(HookError::new("Pre-commit")),
				}
			}
			false => Ok(()),
		};
	}

	pub fn commit_msg(&self, editmsg_path: PathBuf) -> Result<(), Box<dyn Error>> {
		let p = PathBuf::from(format!("{}/commit-msg", self.path));
		return match p.exists() {
			true => {
				let status = Command::new(p).arg(editmsg_path.to_str().unwrap()).status();
				let succeeded = status.is_ok() && status.unwrap().success();

				match succeeded {
					true => Ok(()),
					false => Err(HookError::new("Commit-msg")),
				}
			}
			false => Ok(()),
		};
	}
}

#[cfg(test)]
mod test;