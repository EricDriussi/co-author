use crate::Result;
use crate::{
	args::Args,
	authors::{self, author::Author},
	git::{self, commit_mode::CommitMode},
	ui::cli::Cli,
};

// TODO: eval creating an orchestrator that is constructed with args and cli and has a function for authors and another one for commit_msg
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

fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
	vector.sort();
	vector
}

pub fn handle_commit_msg(args: &Args, cli: &mut Cli, authors_signatures: Vec<String>) -> Result<()> {
	let mut git_service = git::di::init()?;
	let prev = git_service.last_commit_message();

	if args.editor {
		if args.pre_populate {
			return git_service.commit(CommitMode::WithEditor {
				message: Some(prev.as_str()),
				authors: authors_signatures,
			});
		}
		return git_service.commit(CommitMode::WithEditor {
			message: None,
			authors: authors_signatures,
		});
	}

	let msg = match (args.message.clone(), args.pre_populate) {
		(Some(msg), _) => msg,
		(None, false) => cli.prompt_for_message()?,
		(None, true) => cli.prompt_for_pre_populated_message(&prev)?,
	};

	git_service.commit(CommitMode::WithoutEditor {
		message: msg.as_str(),
		authors: authors_signatures,
	})
}
