use crate::Result;
use std::fmt::Display;

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

	pub fn formatted(&self) -> String {
		format!("{}\n\n\n{}", self.subject, self.authors.join("\n"))
	}
}

impl Display for CommitMessage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.formatted())
	}
}

#[cfg_attr(test, mockall::automock)]
pub trait GitWrapper {
	fn commit(&self) -> Result<()>;
	fn formatted_status(&self) -> Result<String>;
	fn prev_commit_msg(&self) -> Result<String>;
}
