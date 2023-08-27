use std::{
	env, fs,
	io::{BufRead, BufReader},
	path::PathBuf,
	process::{Command, Stdio},
};

use git2::Config;

pub fn get_commit_message_from_editor(editmsg: PathBuf) -> Option<String> {
	// FIXME.Propper error handling
	// FIXME.COMMIT_EDITMSG needs to be pre-populated with the output of "git status" as comments, simulating default git behavior

	match default_editor() {
		Some(editor) => Command::new(&editor)
			.arg(editmsg.as_path())
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.output()
			.unwrap(),
		None => match Command::new("vim")
			.arg(editmsg.as_path())
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.output()
		{
			Ok(output) => Ok(output),
			Err(_) => Command::new("vi")
				.arg(editmsg.as_path())
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.output(),
		}
		.unwrap(),
	};

	let file = fs::File::open(editmsg.as_path()).unwrap();
	let reader = BufReader::new(file);
	let mut message = String::new();

	for line in reader.lines() {
		let line = line.unwrap();
		if !line.starts_with('#') {
			message.push_str(&line.trim());
			message.push('\n');
		}
	}

	let trimmed_message = message.trim();
	return if trimmed_message.is_empty() {
		None
	} else {
		Some(trimmed_message.to_string())
	};
}

fn default_editor() -> Option<String> {
	let config = Config::open_default().unwrap();
	match config.get_string("core.editor") {
		Ok(editor) => Some(editor),
		Err(_) => match env::var("EDITOR") {
			Ok(editor) => Some(editor),
			Err(_) => None,
		},
	}
}

#[cfg(test)]
mod test;
