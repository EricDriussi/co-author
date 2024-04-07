use crate::error::Error;
use std::{any::Any, fmt::Display};

#[derive(Debug)]
pub enum SystemError {
	Runner(String, String),
	Read(String),
	Write(String),
	EnvVar(String),
}

impl Error for SystemError {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl std::error::Error for SystemError {}

impl PartialEq for SystemError {
	fn eq(&self, other: &Self) -> bool {
		matches!((self, other), |(
			SystemError::Runner(_, _),
			SystemError::Runner(_, _),
		)| (SystemError::Read(_), SystemError::Read(_))
			| (SystemError::Write(_), SystemError::Write(_))
			| (SystemError::EnvVar(_), SystemError::EnvVar(_)))
	}
}

impl Display for SystemError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "System failure: ")?;
		match self {
			SystemError::Runner(cmd, err) => write!(f, "Command {cmd} failed with: {err}"),
			SystemError::Read(err) => write!(f, "Could not read {err}"),
			SystemError::Write(err) => write!(f, "Could not write {err}"),
			SystemError::EnvVar(var) => write!(f, "Could not get env var {var}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_system_error_display() {
		assert_eq!(
			format!("{}", SystemError::Runner("cmd".to_string(), "error".to_string())),
			"System failure: Command cmd failed with: error"
		);
		assert_eq!(
			format!("{}", SystemError::Read("file".to_string())),
			"System failure: Could not read file"
		);
		assert_eq!(
			format!("{}", SystemError::Write("file".to_string())),
			"System failure: Could not write file"
		);
		assert_eq!(
			format!("{}", SystemError::EnvVar("whatever".to_string())),
			"System failure: Could not get env var whatever"
		);
	}
}
