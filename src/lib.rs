use std::io::{BufRead, Write};

use authors::author::Author;
use authors::{app_service::AuthorsService, author::AuthorsRepo};
use cli::Cli;
use git::{app_service::GitService, git::GitRepo};

pub mod cli;

pub fn run_interactive<T: GitRepo, Y: AuthorsRepo, R: BufRead, W: Write>(
	git_service: GitService<T>,
	auth_service: AuthorsService<Y>,
	mut cli: Cli<R, W>,
) -> Result<(), String> {
	print(auth_service.all_available());
	let aliases = cli.ask_for_aliases();
	let found_authors = auth_service.signatures_of(aliases);
	let commit_body = cli.ask_for_commit_message()?;

	return git_service.commit(commit_body.as_str(), found_authors);
}

fn print(authors: Vec<Author>) {
	println!();
	for author in &authors {
		println!("{}", author);
	}
	println!();
}
