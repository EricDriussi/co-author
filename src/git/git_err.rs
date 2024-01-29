use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GitError {
	Hook,
	Editor,
}

impl Error for GitError {}

impl Display for GitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "GIT failure: ")?;
		match self {
			GitError::Hook => write!(f, "Hook"),
			GitError::Editor => write!(f, "Editor"),
		}
	}
}
