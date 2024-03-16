use crate::{
	args::Args,
	authors::{
		author::Author,
		di::{init_authors_module, init_authors_module_for},
	},
	cli::prompt::Prompt,
	Result,
};

pub fn handle_authors(args: &Args, cli: &mut Prompt) -> Result<Vec<String>> {
	let authors_prov = match &args.file {
		Some(file) => init_authors_module_for(file)?,
		None => init_authors_module()?,
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

	let aliases = cli.prompt_aliases(&authors_prov.all())?;
	if args.sort {
		Ok(sort(authors_prov.find(aliases).iter().map(Author::signature).collect()))
	} else {
		Ok(authors_prov.find(aliases).iter().map(Author::signature).collect())
	}
}

pub fn handle_commit_msg(args: &Args, cli: &mut Prompt, prev: &str) -> Result<String> {
	match (args.message.clone(), args.pre_populate) {
		(Some(msg), _) => Ok(msg),
		(None, false) => cli.prompt_commit_message(),
		(None, true) => cli.prompt_pre_populated_commit_message(prev),
	}
}

// TODO: Don't mutate
pub fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
	vector.sort();
	vector
}
