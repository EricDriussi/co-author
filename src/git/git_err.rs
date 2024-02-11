use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GitError {
	Hook(String),
	Editor,
}

impl Error for GitError {}

impl Display for GitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "GIT failure: ")?;
		match self {
			GitError::Hook(hook) => write!(f, "Hook: {hook}"),
			GitError::Editor => write!(f, "Editor"),
		}
	}
}
