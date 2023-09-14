use std::{error::Error, path::PathBuf, process::Command};

use crate::git_err::HookError;

pub fn pre_commit(mut hooks_path: PathBuf) -> Result<(), Box<dyn Error>> {
	hooks_path.push("pre-commit");
	return match hooks_path.exists() {
		true => {
			let status = Command::new(&hooks_path).status();
			println!("{:?}", status);
			let succeeded = status.is_ok() && status.unwrap().success();

			match succeeded {
				true => Ok(()),
				false => Err(HookError::new("Pre-commit")),
			}
		}
		false => Ok(()),
	};
}

pub fn commit_msg(mut hooks_path: PathBuf, editmsg_path: PathBuf) -> Result<(), Box<dyn Error>> {
	hooks_path.push("commit-msg");
	return match hooks_path.exists() {
		true => {
			let status = Command::new(&hooks_path).arg(editmsg_path.to_str().unwrap()).status();
			let succeeded = status.is_ok() && status.unwrap().success();

			match succeeded {
				true => Ok(()),
				false => Err(HookError::new("Commit-msg")),
			}
		}
		false => Ok(()),
	};
}

#[cfg(test)]
mod test;
