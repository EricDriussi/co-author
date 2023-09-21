use std::error::Error;

use args::Args;
use cli::Cli;

pub mod args;
pub mod cli;

pub fn handle_authors(args: &Args, cli: &mut impl Cli) -> Result<Vec<String>, Box<dyn Error>> {
	let authors_service = match &args.file {
		Some(file) => authors::fs_setup_from_file(file.to_string())?,
		None => authors::fs_default_setup(conf::authors_file())?,
	};

	if args.all {
		return match args.sort {
			true => Ok(sort(authors_service.all_signatures())),
			false => Ok(authors_service.all_signatures()),
		};
	}
	if let Some(list) = &args.list {
		let given_aliases = list.split(',').map(|alias| alias.to_string()).collect();
		return match args.sort {
			true => Ok(sort(authors_service.signatures_of(given_aliases))),
			false => Ok(authors_service.signatures_of(given_aliases)),
		};
	}

	let aliases = cli.ask_for_aliases(authors_service.all_available())?;
	return match args.sort {
		true => Ok(sort(authors_service.signatures_of(aliases))),
		false => Ok(authors_service.signatures_of(aliases)),
	};
}

pub fn handle_commit_msg(args: &Args, cli: &mut impl Cli, prev: String) -> Result<String, Box<dyn Error>> {
	if args.message.is_some() {
		return Ok(args.message.clone().unwrap());
	}
	return match args.pre_populate {
		false => cli.ask_for_commit_message(),
		true => cli.ask_for_commit_message_with_pre_populated(prev),
	};
}

pub fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
	vector.sort();
	vector
}
