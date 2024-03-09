use super::err::CliError;
use crate::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

#[cfg_attr(test, mockall::automock)]
pub trait Reader {
	fn readline(&mut self, prompt_msg: &str) -> Result<String>;
	fn readline_with_initial<'a>(&mut self, prompt: &str, initial: (&'a str, &'a str)) -> Result<String>;
}

impl Reader for DefaultEditor {
	fn readline(&mut self, prompt_msg: &str) -> Result<String> {
		Ok(self.readline(prompt_msg).map_err(|e| match e {
			ReadlineError::Interrupted => CliError::Interrupted,
			ReadlineError::Io(e) => CliError::Io(e),
			_ => CliError::Unknown,
		})?)
	}

	fn readline_with_initial(&mut self, prompt_msg: &str, pre_populate: (&str, &str)) -> Result<String> {
		Ok(self
			.readline_with_initial(prompt_msg, pre_populate)
			.map_err(|e| match e {
				ReadlineError::Interrupted => CliError::Interrupted,
				ReadlineError::Io(e) => CliError::Io(e),
				_ => CliError::Unknown,
			})?)
	}
}
