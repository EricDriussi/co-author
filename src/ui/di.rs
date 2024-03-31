use super::{cli::Cli, err::UiError};
use crate::Result;

pub fn init() -> Result<Cli> {
	Ok(Cli::new(Box::new(
		rustyline::DefaultEditor::new().map_err(|_| UiError::Unknown)?,
	)))
}
