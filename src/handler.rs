use crate::{
	args::Args,
	authors::{self, author::Author},
	ui::cli::Cli,
	Result,
};

pub fn handle_authors(args: &Args, cli: &mut Cli) -> Result<Vec<String>> {
	let authors_prov = match &args.file {
		Some(file) => authors::di::init_for(file)?,
		None => authors::di::init()?,
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

	let aliases = cli.prompt_for_aliases(&authors_prov.all())?;
	if args.sort {
		Ok(sort(authors_prov.find(aliases).iter().map(Author::signature).collect()))
	} else {
		Ok(authors_prov.find(aliases).iter().map(Author::signature).collect())
	}
}

pub fn handle_commit_msg(args: &Args, cli: &mut Cli, prev: &str) -> Result<String> {
	match (args.message.clone(), args.pre_populate) {
		(Some(msg), _) => Ok(msg),
		(None, false) => cli.prompt_for_message(),
		(None, true) => cli.prompt_for_pre_populated_message(prev),
	}
}

fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
	vector.sort();
	vector
}
