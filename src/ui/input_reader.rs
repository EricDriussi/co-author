use super::err::UiError;
use crate::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

#[cfg_attr(test, mockall::automock)]
pub trait InputReader {
	fn readline(&mut self, prompt: &str) -> Result<String>;
	fn readline_with_prompt<'a>(&mut self, prompt: &str, initial: (&'a str, &'a str)) -> Result<String>;
}

impl InputReader for DefaultEditor {
	fn readline(&mut self, prompt: &str) -> Result<String> {
		Ok(self.readline(&format!("{prompt}> ")).map_err(|e| match e {
			ReadlineError::Interrupted => UiError::Interrupted,
			ReadlineError::Io(e) => UiError::Io(e),
			e => UiError::Unknown(e.to_string()),
		})?)
	}

	fn readline_with_prompt(&mut self, prompt: &str, pre_populate: (&str, &str)) -> Result<String> {
		Ok(self
			.readline_with_initial(&format!("{prompt}> "), pre_populate)
			.map_err(|e| match e {
				ReadlineError::Interrupted => UiError::Interrupted,
				ReadlineError::Io(e) => UiError::Io(e),
				e => UiError::Unknown(e.to_string()),
			})?)
	}
}
