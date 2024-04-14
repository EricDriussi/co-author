use super::err::SystemError;
use crate::Result;
use std::process::{Child, Command, Stdio};

#[cfg_attr(test, mockall::automock)]
pub trait Runner {
	fn run(&self, cmd: &str, arg: &str) -> Result<()>;
	fn spawn(&self, editor: &str, arg: &str) -> Result<()>;
	fn attach(&self, cmd: &str, args: &[String]) -> Result<Child>;
}

pub struct CommandRunner;

impl Runner for CommandRunner {
	fn spawn(&self, cmd: &str, arg: &str) -> Result<()> {
		Ok(Command::new(cmd)
			.arg(arg)
			.spawn()
			.map(|mut child| (child.wait()))
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

	fn attach(&self, cmd: &str, args: &[String]) -> Result<Child> {
		Ok(Command::new(cmd)
			.args(args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()
			.map_err(|e| SystemError::Runner(cmd.to_string(), e.to_string()))?)
	}
}
