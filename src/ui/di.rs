use super::cli::Cli;
use crate::Result;

pub fn init() -> Result<Cli> {
	Ok(Cli::new(Box::new(rustyline::DefaultEditor::new()?)))
}
