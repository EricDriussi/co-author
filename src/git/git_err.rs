use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GitError {
	Editor,
	Hook(String),
	LibGit(String),
	Editmsg,
}

impl Error for GitError {}

impl Display for GitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "GIT failure: ")?;
		match self {
			GitError::Editor => write!(f, "Editor"),
			GitError::Hook(hook) => write!(f, "Hook: {hook}"),
			GitError::LibGit(err) => write!(f, "{err}"),
			GitError::Editmsg => write!(f, "EDITMSG file not found"),
		}
	}
}
