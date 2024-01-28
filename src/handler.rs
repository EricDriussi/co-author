use crate::{
	args::Args,
	authors::{
		author::{Author, AuthorsProvider},
		csv::provider::CSVReader,
	},
	cli::{input_reader::Reader, prompt::Prompt},
	fs::wrapper::FsWrapper,
	Result,
};

pub fn handle_authors<T: Reader>(args: &Args, cli: &mut Prompt<T>) -> Result<Vec<String>> {
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

	let aliases = cli.prompt_aliases(&authors_prov.all())?;
	if args.sort {
		Ok(sort(authors_prov.find(aliases).iter().map(Author::signature).collect()))
	} else {
		Ok(authors_prov.find(aliases).iter().map(Author::signature).collect())
	}
}

pub fn handle_commit_msg<T: Reader>(args: &Args, cli: &mut Prompt<T>, prev: &str) -> Result<String> {
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
