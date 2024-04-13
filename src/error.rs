use crate::{authors::err::AuthorsError, common::err::SystemError, git::err::GitError, ui::err::UiError};
use std::any::Any;

pub trait Error: std::error::Error + Any {
	fn as_any(&self) -> &dyn Any;
}

impl From<UiError> for Box<dyn Error> {
	fn from(e: UiError) -> Box<dyn Error> {
		Box::new(e)
	}
}

impl From<AuthorsError> for Box<dyn Error> {
	fn from(e: AuthorsError) -> Box<dyn Error> {
		Box::new(e)
	}
}

impl From<GitError> for Box<dyn Error> {
	fn from(e: GitError) -> Box<dyn Error> {
		Box::new(e)
	}
}

impl From<SystemError> for Box<dyn Error> {
	fn from(e: SystemError) -> Box<dyn Error> {
		Box::new(e)
	}
}

impl From<git2::Error> for Box<dyn Error> {
	fn from(e: git2::Error) -> Box<dyn Error> {
		Box::new(GitError::LibGit(e.to_string()))
	}
}

#[cfg(test)]
pub fn assert_error_type<T, E: Error + 'static + PartialEq>(result: &crate::Result<T>, expected_error: &E) {
	assert!(result.is_err(), "Not an Error");
	assert!(
		matches!(result, Err(ref e) if e.as_any().downcast_ref::<E>().map_or(false, |err| *err == *expected_error)),
		"Expected error type: {:?}, but got: {:?}",
		expected_error,
		result.as_ref().err(),
	);
}

#[cfg(test)]
pub fn assert_error_contains_msg<T>(result: &crate::Result<T>, expected_msg: &str) {
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
	use std::{fmt::Display, io};

	#[derive(Debug)]
	enum TestError {
		Unknown(String),
	}

	impl Error for TestError {
		fn as_any(&self) -> &dyn Any {
			self
		}
	}

	impl std::error::Error for TestError {}

	impl PartialEq for TestError {
		fn eq(&self, other: &Self) -> bool {
			matches!(self, TestError::Unknown(_)) && matches!(other, TestError::Unknown(_))
		}
	}

	impl Display for TestError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "Test error: ")?;
			match self {
				TestError::Unknown(err) => write!(f, "{err}"),
			}
		}
	}

	#[cfg(test)]
	impl From<&str> for Box<dyn Error> {
		fn from(e: &str) -> Box<dyn Error> {
			Box::new(TestError::Unknown(e.to_string()))
		}
	}

	#[cfg(test)]
	impl From<Box<dyn std::error::Error>> for Box<dyn Error> {
		fn from(value: Box<dyn std::error::Error>) -> Self {
			Box::new(TestError::Unknown(value.to_string()))
		}
	}

	#[cfg(test)]
	impl From<io::Error> for Box<dyn Error> {
		fn from(e: io::Error) -> Box<dyn Error> {
			Box::new(TestError::Unknown(e.to_string()))
		}
	}

	#[cfg(test)]
	impl From<String> for Box<dyn Error> {
		fn from(e: String) -> Box<dyn Error> {
			Box::new(TestError::Unknown(e))
		}
	}
}
