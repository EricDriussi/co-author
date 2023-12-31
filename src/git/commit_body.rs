use std::{error::Error, fmt::Display};

pub struct CommitBody {
	message: String,
	signatures: Vec<String>,
}

impl CommitBody {
	pub fn new(message: &str, signatures: Vec<String>) -> Self {
		Self {
			message: String::from(message),
			signatures,
		}
	}

	pub fn formatted_body(&self) -> String {
		format!(
			"{}\n\n\n{}",
			self.message,
			self.signatures
				.join("\n")
		)
	}
}

impl Display for CommitBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\n\n{}",
			self.message,
			self.signatures
				.join("\n")
		)
	}
}

pub trait GitWrapper {
	fn commit(&self) -> Result<(), Box<dyn Error>>;
	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>>;
	fn write_to_editmsg(&self, commit_body: CommitBody) -> Result<(), Box<dyn Error>>;
	fn prev_commit_msg(&self) -> Result<String, Box<dyn Error>>;
}
