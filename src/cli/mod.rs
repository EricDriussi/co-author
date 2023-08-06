use std::{
	env, fs,
	io::{BufRead, BufReader, Write},
	path::Path,
	process::{Command, Stdio},
};

mod reader;

pub struct Cli<R: BufRead, W: Write> {
	input: R,
	output: W,
}

impl<R: BufRead, W: Write> Cli<R, W> {
	pub fn new(input: R, output: W) -> Self {
		Cli { input, output }
	}

	pub fn ask_for_commit_message(&mut self) -> Result<String, &'static str> {
		let commit_message = reader::prompt("Enter your commit message:", &mut self.input, &mut self.output);

		if commit_message.is_empty() {
			return Err("Commit message cannot be empty.");
		}
		return Ok(commit_message);
	}

	pub fn ask_for_aliases(&mut self) -> Vec<String> {
		let aliases = reader::prompt(
			"Enter co-authors aliases separated by spaces:",
			&mut self.input,
			&mut self.output,
		);

		return aliases.split_whitespace().map(|s| s.to_string()).collect();
	}

	pub fn get_commit_from_editor() -> Option<String> {
		// FIXME.Propper error handling
		// FIXME.COMMIT_EDITMSG needs to be pre-populated with the output of "git status" as comments, simulating default git behavior
		let commit_editmsg_path = Path::new("./.git/COMMIT_EDITMSG");
		match Self::get_editor() {
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
			if !line.starts_with('#') {
				message.push_str(&line);
				message.push('\n');
			}
		}
		Some(message)
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
}

#[cfg(test)]
mod test;
