use crate::{common::conf, Result};
use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct CommitMessage {
	subject: String,
	body: Vec<String>,
	authors: Vec<String>,
}

impl CommitMessage {
	pub fn new(message: &str, authors: Vec<String>) -> Self {
		let mut lines = message.lines().map(str::trim);
		let subject = lines.next().unwrap_or_default().to_string();
		let body = lines
			.filter(|line| !line.is_empty())
			.map(String::from)
			.collect::<Vec<String>>();

		Self { subject, body, authors }
	}

	pub fn from(message: &str) -> Self {
		let mut non_empty_lines = message
			.lines()
			.map(str::trim)
			.filter(|line| !line.is_empty())
			.filter(|line| !line.starts_with('#'));

		let subject = non_empty_lines.next().unwrap_or_default().to_string();
		let (body, authors) = non_empty_lines
			.map(String::from)
			.partition(|line| !line.starts_with(&conf::co_author_prefix()));

		Self { subject, body, authors }
	}

	pub fn body(&self) -> &Vec<String> {
		&self.body
	}

	pub fn subject(&self) -> &str {
		&self.subject
	}

	pub fn formatted(&self) -> String {
		let body = if self.body.is_empty() {
			String::new()
		} else {
			format!("\n\n{}", self.body.join("\n"))
		};

		let authors = if self.authors.is_empty() {
			String::new()
		} else {
			format!("\n\n\n{}", self.authors.join("\n"))
		};

		format!("{}{}{}", self.subject, body, authors)
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
	fn prev_commit_msg(&self) -> Result<CommitMessage>;
}
