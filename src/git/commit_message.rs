#[cfg(test)]
use mockall::{automock, predicate::*};
use std::{error::Error, fmt::Display};

// TODO: handle subject and body
#[derive(Debug, PartialEq)]
pub struct CommitMessage {
	subject: String,
	// body: Vec<String>,
	authors: Vec<String>,
}

impl CommitMessage {
	pub fn new(message: &str, authors: Vec<String>) -> Self {
		Self {
			subject: String::from(message),
			authors,
		}
	}

	pub fn formatted_body(&self) -> String {
		format!("{}\n\n\n{}", self.subject, self.authors.join("\n"))
	}
}

impl Display for CommitMessage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n\n{}", self.subject, self.authors.join("\n"))
	}
}

#[cfg_attr(test, automock)]
pub trait GitWrapper {
	fn commit(&self) -> Result<(), Box<dyn Error>>;
	fn add_status_to_editmsg(&mut self) -> Result<(), Box<dyn Error>>;
	fn write_to_editmsg(&mut self, commit_message: &CommitMessage) -> Result<(), Box<dyn Error>>;
	fn prev_commit_msg(&self) -> Result<String, Box<dyn Error>>;
}
