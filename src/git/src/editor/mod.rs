use std::{
	env, fs,
	io::{self, BufRead, BufReader},
	path::{Path, PathBuf},
	process::{Command, Output, Stdio},
};

use git2::Config;

pub fn get_commit_message_from_editor(editmsg: PathBuf) -> Option<String> {
	match Config::open_default() {
		Ok(config) => match config.get_string("core.editor") {
			Ok(editor) => match open_editor(&editor, editmsg.as_path()) {
				Ok(_) => return read_message_from_file(editmsg.as_path()),
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

fn env_fallback(path: &Path) -> Option<String> {
	match env::var("EDITOR") {
		Ok(editor) => match open_editor(&editor, path) {
			Ok(_) => return read_message_from_file(path),
			Err(_) => return vim_fallback(path),
		},
		Err(_) => return vim_fallback(path),
	}
}

fn vim_fallback(path: &Path) -> Option<String> {
	match open_editor("vim", path) {
		Ok(_) => return read_message_from_file(path),
		Err(_) => match open_editor("vi", path) {
			Ok(_) => return read_message_from_file(path),
			Err(_) => panic!("No editor available!"),
		},
	}
}

fn read_message_from_file(file_path: &Path) -> Option<String> {
	let file = fs::File::open(file_path).expect("Something went wrong");
	let reader = BufReader::new(file);
	let mut message = String::new();

	for line in reader.lines() {
		if let Ok(line) = line {
			if !line.starts_with('#') {
				message.push_str(&line.trim());
				message.push('\n');
			}
		}
	}

	let trimmed_message = message.trim();
	if trimmed_message.is_empty() {
		None
	} else {
		Some(trimmed_message.to_string())
	}
}

#[cfg(test)]
mod test;
