use std::error::Error;

use crate::{
	args::Args,
	authors::{
		author::{Author, AuthorsProvider},
		csv_provider::provider::CSVReader,
	},
	cli::Cli,
	fs::wrapper::FsWrapper,
};

pub fn handle_authors(args: &Args, cli: &mut impl Cli) -> Result<Vec<String>, Box<dyn Error>> {
	let authors_prov = match &args.file {
		Some(file) => CSVReader::from(&FsWrapper::new(), file)?,
		None => CSVReader::from_cwd_fallback_home(&FsWrapper::new())?,
	};

	if args.all {
		return if args.sort {
			Ok(sort(authors_prov.all().iter().map(Author::signature).collect()))
		} else {
			Ok(authors_prov.all().iter().map(Author::signature).collect())
		};
	}
	if let Some(list) = &args.list {
		let given_aliases = list.split(',').map(ToString::to_string).collect();
		return if args.sort {
			Ok(sort(
				authors_prov.find(given_aliases).iter().map(Author::signature).collect(),
			))
		} else {
			Ok(authors_prov.find(given_aliases).iter().map(Author::signature).collect())
		};
	}

	let aliases = cli.ask_for_aliases(authors_prov.all())?;
	if args.sort {
		Ok(sort(authors_prov.find(aliases).iter().map(Author::signature).collect()))
	} else {
		Ok(authors_prov.find(aliases).iter().map(Author::signature).collect())
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
