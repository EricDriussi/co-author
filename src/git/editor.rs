use std::{
	env,
	io::{self},
	path::{Path, PathBuf},
	process::{Command, Output, Stdio},
};

use co_author::conf;
use git2::Config;

pub fn open() {
	let editmsg = PathBuf::from(conf::editmsg());
	match Config::open_default() {
		Ok(config) => match config.get_string("core.editor") {
			Ok(editor) => match open_editor(&editor, editmsg.as_path()) {
				Ok(_) => (),
				Err(_) => return env_fallback(editmsg.as_path()),
			},
			Err(_) => return env_fallback(editmsg.as_path()),
		},
		Err(_) => return env_fallback(editmsg.as_path()),
	}
}

fn open_editor(editor: &str, path: &Path) -> io::Result<Output> {
	return Command::new(editor)
		.arg(path)
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.output();
}

fn env_fallback(path: &Path) {
	match env::var("EDITOR") {
		Ok(editor) => match open_editor(&editor, path) {
			Ok(_) => (),
			Err(_) => vim_fallback(path),
		},
		Err(_) => vim_fallback(path),
	}
}

fn vim_fallback(path: &Path) {
	open_editor("vim", path)
		.or_else(|_| open_editor("vi", path))
		.expect("No editor available!");
}
