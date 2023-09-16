use rustyline::{error::ReadlineError, DefaultEditor};

pub struct Cli {
	reader: DefaultEditor,
}

impl Cli {
	pub fn new() -> Self {
		Self {
			reader: DefaultEditor::new().unwrap(),
		}
	}

	pub fn ask_for_commit_message(&mut self) -> Result<String, &'static str> {
		match self.reader.readline("Enter your commit message:\n") {
			Ok(commit_message) => return Cli::process_commit_msg(commit_message),
			//FIXME. Add Cli error?
			Err(_) => return Err("GENERIC ERROR"),
		};
	}

	pub fn ask_for_commit_message_with_prev(&mut self, prev_commit_msg: String) -> Result<String, &'static str> {
		match self
			.reader
			.readline_with_initial("Enter your commit message:\n", (prev_commit_msg.as_str(), ""))
		{
			Ok(commit_message) => return Cli::process_commit_msg(commit_message),
			Err(_) => return Err("GENERIC ERROR"),
		}
	}

	fn process_commit_msg(msg: String) -> Result<String, &'static str> {
		let trimmed_msg = msg.trim().to_string();
		Cli::validate_commit_msg(trimmed_msg)
	}

	fn validate_commit_msg(msg: String) -> Result<String, &'static str> {
		match msg.is_empty() {
			false => Ok(msg),
			true => Err("Commit message cannot be empty."),
		}
	}

	pub fn ask_for_aliases(&mut self) -> Result<Vec<String>, &'static str> {
		match self.reader.readline("Enter co-authors aliases separated by spaces:") {
			Ok(aliases) => return Ok(Cli::process_aliases(aliases)),
			Err(_) => return Err("GENERIC ERROR"),
		}
	}

	fn process_aliases(aliases: String) -> Vec<String> {
		aliases.split_whitespace().map(|s| s.to_string()).collect()
	}
}

#[cfg(test)]
mod test;
