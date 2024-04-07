use crate::error::Error;
use std::{any::Any, fmt::Display, io};

#[derive(Debug)]
pub enum UiError {
	Io(io::Error),
	Interrupted,
	Unknown(String),
}

impl Error for UiError {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl std::error::Error for UiError {}

impl PartialEq for UiError {
	fn eq(&self, other: &Self) -> bool {
		matches!(
			(self, other),
			(UiError::Io(_), UiError::Io(_))
				| (UiError::Interrupted, UiError::Interrupted)
				| (UiError::Unknown(_), UiError::Unknown(_))
		)
	}
}

impl Display for UiError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Cli failure: ")?;
		match self {
			UiError::Io(ref err) => err.fmt(f),
			UiError::Interrupted => write!(f, "Interrupted"),
			UiError::Unknown(err) => write!(f, "{err}"),
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
		assert_eq!(format!("{}", UiError::Unknown("oops".to_string())), "Cli failure: oops");

		let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
		let cli_error = UiError::Io(io_error);
		assert!(format!("{cli_error}").contains("Cli failure: "));
		assert!(format!("{cli_error}").contains("file not found"));
	}
}
