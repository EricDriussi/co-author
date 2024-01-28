use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AuthorsError {
	NotFound(String),
}

impl Error for AuthorsError {}

impl Display for AuthorsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Authors error: ")?;
		match self {
			AuthorsError::NotFound(location) => write!(f, "No file at {location}"),
		}
	}
}
