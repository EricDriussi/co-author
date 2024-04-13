use crate::error::Error;
use std::{any::Any, fmt::Display};

#[derive(Debug)]
pub enum GitError {
	Editor,
	Hook(String),
	LibGit(String),
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
				| (GitError::Hook(_), GitError::Hook(_))
				| (GitError::LibGit(_), GitError::LibGit(_))
		)
	}
}

impl Display for GitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Git: ")?;
		match self {
			GitError::Editor => write!(f, "Editor"),
			GitError::Hook(hook) => write!(f, "{hook} hook"),
			GitError::LibGit(err) => write!(f, "{err}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_git_error_display() {
		assert_eq!(format!("{}", GitError::Editor), "Git: Editor");
		assert_eq!(
			format!("{}", GitError::Hook("pre-commit".to_string())),
			"Git: pre-commit hook"
		);
		assert_eq!(
			format!("{}", GitError::LibGit("some error".to_string())),
			"Git: some error"
		);
	}
}
