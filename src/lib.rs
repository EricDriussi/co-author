use std::error::Error;

use args::Args;
use cli::Cli;
pub mod authors;
pub mod conf;
pub mod fs_wrapper;
pub mod test_utils;

pub mod args;
pub mod cli;

pub fn handle_authors(args: &Args, cli: &mut impl Cli) -> Result<Vec<String>, Box<dyn Error>> {
	let authors_service = match &args.file {
		Some(file) => authors::from_file(file)?,
		None => authors::default()?,
	};

	if args.all {
		return if args.sort {
			Ok(sort(authors_service.all_signatures()))
		} else {
			Ok(authors_service.all_signatures())
		};
	}
	if let Some(list) = &args.list {
		let given_aliases = list.split(',').map(ToString::to_string).collect();
		return if args.sort {
			Ok(sort(authors_service.signatures_of(given_aliases)))
		} else {
			Ok(authors_service.signatures_of(given_aliases))
		};
	}

	let aliases = cli.ask_for_aliases(authors_service.all_authors())?;
	if args.sort {
		Ok(sort(authors_service.signatures_of(aliases)))
	} else {
		Ok(authors_service.signatures_of(aliases))
	}
}

pub fn handle_commit_msg(args: &Args, cli: &mut impl Cli, prev: String) -> Result<String, Box<dyn Error>> {
	match (args.message.clone(), args.pre_populate) {
		(Some(msg), _) => Ok(msg),
		(None, false) => cli.ask_for_commit_message(),
		(None, true) => cli.ask_for_commit_message_with_pre_populated(prev),
	}
}

// TODO: Don't mutate
pub fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
	vector.sort();
	vector
}
