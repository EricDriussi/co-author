#[cfg(test)]
use mockall::{automock, predicate::*};
use rustyline::{error::ReadlineError, DefaultEditor};

#[cfg_attr(test, automock)]
pub trait InputReader {
	fn readline(&mut self, prompt: &str) -> Result<String, ReadlineError>;
	fn readline_with_initial<'a>(&mut self, prompt: &str, initial: (&'a str, &'a str))
		-> Result<String, ReadlineError>;
}

impl InputReader for DefaultEditor {
	fn readline(&mut self, prompt: &str) -> Result<String, ReadlineError> {
		self.readline(prompt)
	}

	fn readline_with_initial(&mut self, prompt: &str, initial: (&str, &str)) -> Result<String, ReadlineError> {
		self.readline_with_initial(prompt, initial)
	}
}
