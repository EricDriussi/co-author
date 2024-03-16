use super::prompt::Prompt;
use crate::Result;

pub fn init() -> Result<Prompt> {
	Ok(Prompt::new(Box::new(rustyline::DefaultEditor::new()?)))
}
