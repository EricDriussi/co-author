use std::error::Error;

use args::Args;
use authors::author::Author;

pub mod args;
pub mod cli;

pub fn get_commit_message(args: &Args, mut cli: impl cli::Cli) -> Result<String, Box<dyn Error>> {
	if let Some(message) = &args.message {
		return Ok(message.to_string());
	}
	let commit_body = cli.ask_for_commit_message()?;
	Ok(commit_body)
}

pub fn get_authors_signatures(args: &Args, mut cli: impl cli::Cli) -> Result<Vec<String>, Box<dyn Error>> {
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
	let aliases = cli.ask_for_aliases()?;
	return Ok(authors_service.signatures_of(aliases));
}

// TODO: Could this be handled as a prompt by rustyline?
fn print(authors: Vec<Author>) {
	println!();
	for author in &authors {
		println!("{}", author);
	}
	println!();
}
