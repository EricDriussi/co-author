use std::io::{stdin, stdout, BufRead, Write};

use args::Args;
use authors::author::Author;
use authors::{app_service::AuthorsService, author::AuthorsRepo};
use cli::Cli;
use git::{app_service::GitService, git::GitRepo};

pub mod args;
pub mod cli;

// FIXME.rm this
pub fn run_interactive<T: GitRepo, Y: AuthorsRepo, R: BufRead, W: Write>(
	git_service: GitService<T>,
	authors_service: AuthorsService<Y>,
	mut cli: Cli<R, W>,
) -> Result<(), String> {
	print(authors_service.all_available());
	let aliases = cli.ask_for_aliases();
	let found_authors = authors_service.signatures_of(aliases);
	let commit_body = cli.ask_for_commit_message()?;

	return git_service.commit(commit_body.as_str(), found_authors);
}

pub fn exec(commit_body: String, authors_signatures: Vec<String>) -> Result<(), String> {
	let git_service = git::libgit_setup()?;
	return git_service.commit(commit_body.as_str(), authors_signatures);
}

pub fn get_commit_message<R: BufRead, W: Write>(args: &Args, mut cli: Cli<R, W>) -> Result<String, String> {
	if let Some(message) = &args.message {
		return Ok(message.to_string());
	}
	if args.editor {
		return Ok(cli.get_commit_from_editor().unwrap());
	}
	let commit_body = cli.ask_for_commit_message()?;
	Ok(commit_body)
}

pub fn get_authors_signatures<R: BufRead, W: Write>(args: &Args, mut cli: Cli<R, W>) -> Result<Vec<String>, String> {
	let authors_service = match &args.file {
		Some(file) => authors::fs_setup_from_file(file.to_string())?,
		None => authors::fs_default_setup(conf::authors_file())?,
	};

	if args.all {
		return Ok(authors_service.all_signatures());
	}
	if let Some(list) = &args.list {
		let given_aliases = list.split(',').map(|alias| alias.to_string()).collect();
		return Ok(authors_service.signatures_of(given_aliases));
	}

	print(authors_service.all_available());
	let aliases = cli.ask_for_aliases();
	return Ok(authors_service.signatures_of(aliases));
}

fn print(authors: Vec<Author>) {
	println!();
	for author in &authors {
		println!("{}", author);
	}
	println!();
}
