use super::{cli::Cli, err::UiError};
use crate::{common::runner::CommandRunner, Result};

pub fn init() -> Result<Cli> {
	Ok(Cli::new(
		Box::new(rustyline::DefaultEditor::new().map_err(|e| UiError::Unknown(e.to_string()))?),
		Box::new(CommandRunner),
	))
}
