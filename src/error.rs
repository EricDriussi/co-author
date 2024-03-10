use crate::{authors::err::AuthorsError, cli::err::CliError, git::err::GitError};
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
	Authors(AuthorsError),
	Cli(CliError),
	Git(GitError),
}

impl std::error::Error for Error {}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Error::Authors(a), Error::Authors(b)) => a == b,
			(Error::Cli(a), Error::Cli(b)) => a == b,
			(Error::Git(a), Error::Git(b)) => a == b,
			_ => false,
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "=[ERROR]= ")?;
		match self {
			Error::Authors(err) => err.fmt(f),
			Error::Cli(err) => err.fmt(f),
			Error::Git(err) => err.fmt(f),
		}
	}
}

impl From<AuthorsError> for Error {
	fn from(err: AuthorsError) -> Self {
		Error::Authors(err)
	}
}

impl From<CliError> for Error {
	fn from(err: CliError) -> Self {
		Error::Cli(err)
	}
}

impl From<GitError> for Error {
	fn from(err: GitError) -> Self {
		Error::Git(err)
	}
}

#[cfg(test)]
pub fn assert_error_type<T, E: std::error::Error + 'static + PartialEq>(
	result: &Result<T, Box<dyn std::error::Error>>,
	expected_error: &E,
) {
	assert!(result.is_err(), "Not an Error");
	assert!(
		matches!(result, Err(ref e) if e.downcast_ref::<E>().map_or(false, |err| *err == *expected_error)),
		"Expected error type: {:?}, but got: {:?}",
		expected_error,
		result.as_ref().err(),
	);
}

#[cfg(test)]
pub fn assert_error_contains_msg<T>(result: &Result<T, Box<dyn std::error::Error>>, expected_msg: &str) {
	assert!(result.is_err(), "Not an Error");
	assert!(
		matches!(result, Err(e) if e.to_string().contains(expected_msg)),
		"Expected error message to contain: {:?}, but got: {:?}",
		expected_msg,
		result.as_ref().err(),
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{authors::err::AuthorsError, cli::err::CliError, git::err::GitError};

	#[test]
	fn test_error_display() {
		let an_authors_error = AuthorsError::NotFound("path/to/file".to_string());
		assert!(format!("{}", Error::Authors(an_authors_error)).contains("=[ERROR]= Authors failure"));

		let a_cli_error = CliError::Interrupted;
		assert!(format!("{}", Error::Cli(a_cli_error)).contains("=[ERROR]= Cli failure"));

		let a_git_error = GitError::Editor;
		assert!(format!("{}", Error::Git(a_git_error)).contains("=[ERROR]= Git failure"));
	}
}
