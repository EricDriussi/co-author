use std::{env, error::Error, path::PathBuf, process};

use clap::Parser;
use co_author::{args::Args, cli::FancyCli, handle_authors, handle_commit_msg};

// TODO: option to pre-populate with last commit message (--pre-populate), for both prompt(DONE) and default buffer opening
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
	let mut cli = FancyCli::new();

	let authors = handle_authors(&args, &mut cli)?;

	let git_service = git::libgit_setup()?;
	let prev = git_service.last_commit_message();
	let msg = handle_commit_msg(&args, &mut cli, prev);

	return match msg {
		Some(msg) => git_service.commit(msg?.as_str(), authors),
		None => git_service.commit_with_editor(authors),
	};
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
