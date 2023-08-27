use std::{
	env, fs,
	io::{BufRead, BufReader},
	path::Path,
	process::{Command, Stdio},
};

pub fn get_commit_message_from_editor(tmp_file: String) -> Option<String> {
	// FIXME.Propper error handling
	// FIXME.COMMIT_EDITMSG needs to be pre-populated with the output of "git status" as comments, simulating default git behavior

	let relative_tmp_file_path = format!("../../{}", tmp_file); // FIXME.Needs to go to root dir to find file
	let commit_editmsg_path = Path::new(relative_tmp_file_path.as_str());
	match get_editor() {
		Some(editor) => Command::new(&editor)
			.arg(commit_editmsg_path)
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.output()
			.unwrap(),
		None => match Command::new("vim")
			.arg(commit_editmsg_path)
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.output()
		{
			Ok(output) => Ok(output),
			Err(_) => Command::new("vi")
				.arg(commit_editmsg_path)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.output(),
		}
		.unwrap(),
	};

	let file = fs::File::open(commit_editmsg_path).unwrap();
	let reader = BufReader::new(file);
	let mut message = String::new();

	for line in reader.lines() {
		let line = line.unwrap();
		// TODO.Test comment handling
		if !line.starts_with('#') {
			message.push_str(&line);
			message.push('\n');
		}
	}
	return if message.is_empty() { None } else { Some(message) };
}

fn get_editor() -> Option<String> {
	// FIXME.This requires git2-rs
	// let config = Config::open_default().unwrap();
	// match config.get_string("core.editor") {
	// 	Ok(editor) => Some(editor),
	// 	Err(_) => match env::var("EDITOR") {
	// 		Ok(editor) => Some(editor),
	// 		Err(_) => None,
	// 	},
	// }
	match env::var("EDITOR") {
		Ok(editor) => Some(editor),
		Err(_) => None,
	}
}

#[cfg(test)]
mod test;
