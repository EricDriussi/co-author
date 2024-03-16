use super::prompt::Prompt;
use crate::Result;

pub fn init_cli_module() -> Result<Prompt> {
	Ok(Prompt::new(Box::new(rustyline::DefaultEditor::new()?)))
}
