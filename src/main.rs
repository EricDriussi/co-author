use std::{
	io::{stdin, stdout},
	process,
};

use clap::Parser;
use co_author::{args::Args, cli::Cli, get_authors_signatures, get_commit_message};

// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and default buffer opening
// TODO: sort authors by name when printing
// TODO: automatically create aliases for authors
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

fn run(args: Args) -> Result<(), String> {
	let git_service = git::libgit_setup()?;

	let cli = Cli::new(stdin().lock(), stdout().lock());
	let authors = get_authors_signatures(&args, cli)?;

	if args.editor {
		return git_service.commit_with_editor(authors);
	}

	let cli = Cli::new(stdin().lock(), stdout().lock());
	let commit_body = get_commit_message(&args, cli)?;
	return git_service.commit(commit_body.as_str(), authors);
}
