use crate::Result;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::process::Command;

#[cfg_attr(test, automock)]
pub trait Runner {
	fn run(&self, cmd: &str, arg: &str) -> Result<()>;
	fn spawn(&self, editor: &str, path: &str) -> Result<()>;
}

pub struct CommandRunner {}

impl CommandRunner {
	pub fn new() -> Self {
		Self {}
	}
}

impl Runner for CommandRunner {
	fn spawn(&self, cmd: &str, arg: &str) -> Result<()> {
		Ok(Command::new(cmd).arg(arg).spawn().map(|_| ())?)
	}

	fn run(&self, cmd: &str, arg: &str) -> Result<()> {
		Command::new(cmd)
			.arg(arg)
			.status()?
			.success()
			.then_some(())
			.ok_or("Command returned error code".into())
	}
}
