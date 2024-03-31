use crate::error::Error;
use std::{any::Any, fmt::Display};

#[derive(Debug)]
pub enum GitError {
	Editor,
	Hook(String),
	LibGit(String),
	Editmsg,
	InvalidRepo,
}

impl Error for GitError {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl std::error::Error for GitError {}

impl PartialEq for GitError {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(GitError::Editor, GitError::Editor)
				| (GitError::Editmsg, GitError::Editmsg)
				| (GitError::Hook(_), GitError::Hook(_))
				| (GitError::LibGit(_), GitError::LibGit(_))
				| (GitError::InvalidRepo, GitError::InvalidRepo)
		)
	}
}

impl Display for GitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Git failure: ")?;
		match self {
			GitError::Editor => write!(f, "Editor"),
			GitError::Hook(hook) => write!(f, "Hook: {hook}"),
			GitError::LibGit(err) => write!(f, "{err}"),
			GitError::Editmsg => write!(f, "EDITMSG file not available"),
			GitError::InvalidRepo => write!(f, "Not in a valid git repo"),
		}
	}
}

impl From<git2::Error> for GitError {
	fn from(err: git2::Error) -> Self {
		GitError::LibGit(err.to_string())
	}
}

#[cfg(test)] // simplify errors in tests
impl From<&str> for GitError {
	fn from(err: &str) -> Self {
		GitError::LibGit(err.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_git_error_display() {
		assert_eq!(format!("{}", GitError::Editor), "Git failure: Editor");
		assert_eq!(
			format!("{}", GitError::Hook("pre-commit".to_string())),
			"Git failure: Hook: pre-commit"
		);
		assert_eq!(
			format!("{}", GitError::LibGit("some error".to_string())),
			"Git failure: some error"
		);
		assert_eq!(
			format!("{}", GitError::Editmsg),
			"Git failure: EDITMSG file not available"
		);
		assert_eq!(
			format!("{}", GitError::InvalidRepo),
			"Git failure: Not in a valid git repo"
		);
	}
}
