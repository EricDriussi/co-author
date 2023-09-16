use std::io::{BufRead, Write};

use rustyline::error::ReadlineError;

mod reader;

pub struct Cli<R: BufRead, W: Write> {
	input: R,
	output: W,
}

impl<R: BufRead, W: Write> Cli<R, W> {
	pub fn new(input: R, output: W) -> Self {
		Cli { input, output }
	}

	pub fn ask_for_commit_message2(prev_commit_msg: String) -> Result<String, &'static str> {
		let mut rl = rustyline::DefaultEditor::new().unwrap();
		let input = rl.readline_with_initial("Enter your commit message:\n", (prev_commit_msg.as_str(), ""));
		return Cli::<R, W>::check_for_empty_line(input);
	}

	fn check_for_empty_line(input: Result<String, ReadlineError>) -> Result<String, &'static str> {
		match input {
			Ok(commit_message) => {
				if commit_message.trim().is_empty() {
					Err("Commit message cannot be empty.")
				} else {
					Ok(commit_message)
				}
			}
			Err(ReadlineError::Interrupted) => Err("CTRL C"),
			Err(ReadlineError::Eof) => Err("CTRL D"),
			Err(_) => Err("GENERIC ERROR"),
		}
	}

	pub fn ask_for_commit_message(&mut self) -> Result<String, &'static str> {
		let commit_message = reader::prompt("Enter your commit message:", &mut self.input, &mut self.output);

		if commit_message.is_empty() {
			return Err("Commit message cannot be empty.");
		}
		return Ok(commit_message);
	}

	pub fn ask_for_aliases(&mut self) -> Vec<String> {
		let aliases = reader::prompt(
			"Enter co-authors aliases separated by spaces:",
			&mut self.input,
			&mut self.output,
		);

		return aliases.split_whitespace().map(|s| s.to_string()).collect();
	}
}

#[cfg(test)]
mod test;
