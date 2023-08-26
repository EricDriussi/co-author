use std::{
	io::{stdin, stdout},
	process,
};

use clap::Parser;
use co_author::{args::Args, cli::Cli, exec, get_authors_signatures, get_commit_message};

// TODO: ensure hooks work!
// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and default buffer opening
// TODO: sort authors by name when printing
// TODO: automatically create aliases for authors
// TODO: add amend option -> adds authors to last commit (no message)
// TODO: who should handle empty commit msgs? co-author or libgit2?
// Simpler code if left to libgit2, why handle it in co-author?
// What is libgit2's behavior when empty commit msg?
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
	let cli = Cli::new(stdin().lock(), stdout().lock());
	let authors = get_authors_signatures(&args, cli)?;

	if args.editor {
		let git_service = git::libgit_setup()?;
		return git_service.commit_with_editor(authors);
	}

	let cli = Cli::new(stdin().lock(), stdout().lock());
	let commit_body = get_commit_message(&args, cli)?;
	return exec(commit_body, authors);
}
