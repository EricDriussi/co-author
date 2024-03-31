use crate::error::Error;
use std::{any::Any, fmt::Display};

#[derive(Debug)]
pub enum AuthorsError {
	NotFound(String),
}

impl Error for AuthorsError {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl std::error::Error for AuthorsError {}

impl PartialEq for AuthorsError {
	fn eq(&self, other: &Self) -> bool {
		matches!((self, other), (AuthorsError::NotFound(_), AuthorsError::NotFound(_)))
	}
}

impl Display for AuthorsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Authors failure: ")?;
		match self {
			AuthorsError::NotFound(location) => write!(f, "No file at {location}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_authors_error_display() {
		assert_eq!(
			format!("{}", AuthorsError::NotFound("path/to/file".to_string())),
			"Authors failure: No file at path/to/file"
		);
	}
}
