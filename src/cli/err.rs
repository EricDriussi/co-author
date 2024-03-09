use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum CliError {
	Io(io::Error),
	Interrupted,
	Unknown,
}

impl Error for CliError {}

impl Display for CliError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "CLI failure: ")?;
		match self {
			CliError::Io(ref err) => err.fmt(f),
			CliError::Interrupted => write!(f, "Interrupted"),
			CliError::Unknown => write!(f, "Unknown"),
		}
	}
}
