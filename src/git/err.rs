use crate::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum GitError {
	Editor,
	Hook(String),
	LibGit(String),
	Editmsg,
}

impl Error for GitError {}

impl PartialEq for GitError {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(GitError::Editor, GitError::Editor)
				| (GitError::Editmsg, GitError::Editmsg)
				| (GitError::Hook(_), GitError::Hook(_))
				| (GitError::LibGit(_), GitError::LibGit(_))
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
			GitError::Editmsg => write!(f, "EDITMSG file not found"),
		}
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
		assert_eq!(format!("{}", GitError::Editmsg), "Git failure: EDITMSG file not found");
	}
}
