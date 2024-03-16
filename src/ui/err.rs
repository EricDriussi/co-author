use crate::Error;
use std::{fmt::Display, io};

#[derive(Debug)]
pub enum UiError {
	Io(io::Error),
	Interrupted,
	Unknown,
}

impl Error for UiError {}

impl PartialEq for UiError {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(UiError::Io(_), UiError::Io(_))
				| (UiError::Interrupted, UiError::Interrupted)
				| (UiError::Unknown, UiError::Unknown)
		)
	}
}

impl Display for UiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Cli failure: ")?;
		match self {
			UiError::Io(ref err) => err.fmt(f),
			UiError::Interrupted => write!(f, "Interrupted"),
			UiError::Unknown => write!(f, "Unknown"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::ErrorKind;

	#[test]
	fn test_ui_error_display() {
		assert_eq!(format!("{}", UiError::Interrupted), "Cli failure: Interrupted");
		assert_eq!(format!("{}", UiError::Unknown), "Cli failure: Unknown");

		let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
		let cli_error = UiError::Io(io_error);
		assert!(format!("{cli_error}").contains("Cli failure: "));
		assert!(format!("{cli_error}").contains("file not found"));
	}
}
