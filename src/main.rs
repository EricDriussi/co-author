use std::{env, error::Error, path::PathBuf, process};

use clap::Parser;
use co_author::{args::Args, get_authors_signatures, get_commit_message, new_cli::FancyCli};

// TODO: option to pre-populate with last commit message (--pre-populate), for both prompt(?) and default buffer opening
// TODO: option to sort authors by name when adding to commit message
// TODO: automatically create on the fly aliases for authors
// TODO: add amend option -> update authors of last commit if no message, update message if no authors, normal amend if no message nor author
// TODO: use with fzf or add fuzzy finding

fn main() {
	let args = Args::parse();
	match run(args) {
		Ok(_) => (),
		Err(e) => {
			eprintln!("[Error] {}", e);
			process::exit(1);
		}
	}
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
	set_cwd_to_git_root()?;
	let git_service = git::libgit_setup()?;

	let cli = FancyCli::new();
	//TODO: Single get_commit_data() and pass cli and git_service?
	let authors = get_authors_signatures(&args, cli)?;

	if args.editor {
		return git_service.commit_with_editor(authors);
	}

	let cli = FancyCli::new();
	let commit_body = get_commit_message(&args, cli)?;
	return git_service.commit(commit_body.as_str(), authors);
}

fn set_cwd_to_git_root() -> Result<(), Box<dyn Error>> {
	let project_root_dir = get_project_root_dir()?;
	env::set_current_dir(&project_root_dir).map_err(|_| "Something went wrong")?;
	Ok(())
}

fn get_project_root_dir() -> Result<PathBuf, String> {
	let mut cwd = env::current_dir().map_err(|_| "Something went wrong")?;

	loop {
		let git_dir = cwd.join(".git");
		if git_dir.is_dir() {
			return Ok(cwd);
		}

		if !cwd.pop() {
			break;
		}
	}

	Err("Not in a valid git repo".to_string())
}
