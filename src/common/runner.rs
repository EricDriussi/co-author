use super::err::SystemError;
use crate::Result;
use std::process::Command;

#[cfg_attr(test, mockall::automock)]
pub trait Runner {
	fn run(&self, cmd: &str, arg: &str) -> Result<()>;
	fn spawn(&self, editor: &str, arg: &str) -> Result<()>;
}

pub struct CommandRunner;

impl Runner for CommandRunner {
	fn spawn(&self, cmd: &str, arg: &str) -> Result<()> {
		Ok(Command::new(cmd)
			.arg(arg)
			.spawn()
			.map(|_| ())
			.map_err(|e| SystemError::Runner(cmd.to_string(), e.to_string()))?)
	}

	fn run(&self, cmd: &str, arg: &str) -> Result<()> {
		if Command::new(cmd)
			.arg(arg)
			.status()
			.map_err(|e| SystemError::Runner(cmd.to_string(), e.to_string()))?
			.success()
		{
			Ok(())
		} else {
			Err(SystemError::Runner(cmd.to_string(), "exit code 1".to_string()).into())
		}
	}
}
