use super::prompt::Prompt;
use crate::Result;
use rustyline::{history::FileHistory, Editor};

pub fn init_cli_module() -> Result<Prompt<Editor<(), FileHistory>>> {
	Ok(Prompt::new(rustyline::DefaultEditor::new()?))
}
