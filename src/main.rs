use std::process;

use clap::Parser;
use co_author::{args::Args, exec, get_authors_signatures, get_commit_message};

// TODO: -l and -a should work with -m
// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and default buffer opening
// TODO: sort authors by name when printing
// TODO: automatically create aliases for authors
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
	let authors = get_authors_signatures(&args)?;
	let commit_body = get_commit_message(&args)?;
	return exec(commit_body, authors);
}
