use crate::Error;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum CliError {
	Io(io::Error),
	Interrupted,
	Unknown,
}

impl Error for CliError {}

impl PartialEq for CliError {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(CliError::Io(_), CliError::Io(_))
				| (CliError::Interrupted, CliError::Interrupted)
				| (CliError::Unknown, CliError::Unknown)
		)
	}
}

impl Display for CliError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Cli failure: ")?;
		match self {
			CliError::Io(ref err) => err.fmt(f),
			CliError::Interrupted => write!(f, "Interrupted"),
			CliError::Unknown => write!(f, "Unknown"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::ErrorKind;

	#[test]
	fn test_cli_error_display() {
		assert_eq!(format!("{}", CliError::Interrupted), "Cli failure: Interrupted");
		assert_eq!(format!("{}", CliError::Unknown), "Cli failure: Unknown");

		let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
		let cli_error = CliError::Io(io_error);
		assert!(format!("{cli_error}").contains("Cli failure: "));
		assert!(format!("{cli_error}").contains("file not found"));
	}
}
