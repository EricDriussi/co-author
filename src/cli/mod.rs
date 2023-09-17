use std::{error::Error, process};

use rustyline::{error::ReadlineError::Interrupted, DefaultEditor};

use self::cli_err::CliError;
mod cli_err;

pub struct FancyCli {
	reader: DefaultEditor,
}

pub trait Cli {
	fn ask_for_commit_message(&mut self) -> Result<String, Box<dyn Error>>;
	fn ask_for_aliases(&mut self) -> Result<Vec<String>, Box<dyn Error>>;
}

impl Cli for FancyCli {
	fn ask_for_commit_message(&mut self) -> Result<String, Box<dyn Error>> {
		match self.reader.readline("Enter your commit message:\n") {
			Ok(commit_message) => return FancyCli::process_commit_msg(commit_message),
			Err(Interrupted) => {
				eprintln!("^C");
				process::exit(1);
			}
			Err(_) => return Err(CliError::new("Unexpected error")),
		};
	}

	fn ask_for_aliases(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
		match self.reader.readline("Enter co-authors aliases separated by spaces:") {
			Ok(aliases) => return Ok(FancyCli::process_aliases(aliases)),
			Err(Interrupted) => {
				eprintln!("^C");
				process::exit(1);
			}
			Err(_) => return Err(CliError::new("Unexpected error")),
		}
	}
}

impl FancyCli {
	pub fn new() -> Self {
		Self {
			reader: DefaultEditor::new().unwrap(),
		}
	}

	pub fn ask_for_commit_message_with_prev(&mut self, prev_commit_msg: String) -> Result<String, Box<dyn Error>> {
		match self
			.reader
			.readline_with_initial("Enter your commit message:\n", (prev_commit_msg.as_str(), ""))
		{
			Ok(commit_message) => return FancyCli::process_commit_msg(commit_message),
			Err(_) => return Err(CliError::new("Unexpected error")),
		}
	}

	fn process_commit_msg(msg: String) -> Result<String, Box<dyn Error>> {
		let trimmed_msg = msg.trim().to_string();
		FancyCli::validate_commit_msg(trimmed_msg)
	}

	fn validate_commit_msg(msg: String) -> Result<String, Box<dyn Error>> {
		match msg.is_empty() {
			false => Ok(msg),
			true => Err(CliError::new("Commit message cannot be empty.")),
		}
	}

	fn process_aliases(aliases: String) -> Vec<String> {
		aliases.split_whitespace().map(|s| s.to_string()).collect()
	}
}

#[cfg(test)]
mod test;
