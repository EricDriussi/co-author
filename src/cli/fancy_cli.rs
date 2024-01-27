use colored::Colorize;
use std::{error::Error, process};

use rustyline::error::ReadlineError::{self, Interrupted};

use super::{cli_err::CliError, input_reader::InputReader};
use crate::authors::author::Author;

pub struct FancyCli {
	prompt: Box<dyn InputReader>,
}

impl FancyCli {
	pub fn new(editor: impl InputReader + 'static) -> Self {
		Self {
			prompt: Box::new(editor),
		}
	}

	pub fn prompt_commit_message(&mut self) -> Result<String, Box<dyn Error>> {
		match Self::handle_error(self.prompt.readline("Enter your commit message:\n")) {
			Ok(commit_message) => Ok(commit_message.trim().to_string()),
			Err(e) => Err(e),
		}
	}

	pub fn prompt_aliases(&mut self, authors: &[Author]) -> Result<Vec<String>, Box<dyn Error>> {
		let formatted_authors = Self::prettify_authors(authors);
		let result = self
			.prompt
			.readline((format!("\n{formatted_authors}\n\nEnter co-authors aliases separated by spaces:\n")).as_str());
		match Self::handle_error(result) {
			Ok(aliases) => Ok(aliases.split_whitespace().map(ToString::to_string).collect()),
			Err(e) => Err(e),
		}
	}

	pub fn prompt_pre_populated_commit_message(&mut self, prev_commit_msg: &str) -> Result<String, Box<dyn Error>> {
		let result = self
			.prompt
			.readline_with_initial("Enter your commit message:\n", (prev_commit_msg, ""));
		match Self::handle_error(result) {
			Ok(commit_message) => Ok(commit_message.trim().to_string()),
			Err(e) => Err(e),
		}
	}

	fn handle_error<T>(result: Result<T, ReadlineError>) -> Result<T, Box<dyn Error>> {
		match result {
			Ok(value) => Ok(value),
			Err(Interrupted) => {
				eprintln!("^C");
				process::exit(1);
			}
			Err(_) => Err(CliError::with("Unexpected error")),
		}
	}

	fn prettify_authors(authors: &[Author]) -> String {
		authors.iter().map(Self::prettify).collect::<Vec<String>>().join("\n")
	}

	fn prettify(author: &Author) -> String {
		format!(
			"{} {} {} {}",
			"⦔".yellow(),
			author.alias().blue(),
			"->".green(),
			author.name()
		)
	}
}
