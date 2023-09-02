use std::{fmt::Display, path::PathBuf};

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
		return format!("{}\n\n{}", self.message, self.signatures.join("\n"));
	}
}

impl Display for CommitBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\n\n{}", self.message, self.signatures.join("\n"))
	}
}

pub trait GitWrapper {
	fn commit(&self, body: CommitBody) -> Result<(), String>;
	fn setup_editmsg_file(&self) -> PathBuf;
}
