use args::Args;
use clap::Parser;
use cli::prompt::Prompt;
use git::commit_mode::CommitMode;
use std::{env, error::Error, path::PathBuf, process, result};

// TODO: improve tests
// TODO: review optional/result handling
// TODO: fix bug with first commit in new repo
// TODO: automatically create on the fly aliases for authors
// TODO: add amend option -> update authors if given and update message if given (how does this work with --editor?)
// TODO: use with fzf or add fuzzy finding

fn main() {
	let args = Args::parse();
	if let Err(e) = run(&args) {
		eprintln!("[Error] {e}");
		process::exit(1);
	}
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;
// TODO: use custom error once git module and handler.rs are refactored
// pub type Result<T> = result::Result<T, Error>;

fn run(args: &Args) -> Result<()> {
	set_cwd_to_git_root()?;

	let mut cli = Prompt::new(rustyline::DefaultEditor::new()?);
	let authors_signatures = handler::handle_authors(args, &mut cli)?;

	// FIXME. Find a way to pass this to handle_commit_msg (clone/copy)
	let mut git_service = git::init_git_dependency_tree()?;
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
	let msg = handler::handle_commit_msg(args, &mut cli, &prev)?;

	git_service.commit(CommitMode::WithoutEditor {
		message: msg.as_str(),
		authors: authors_signatures,
	})
}

fn set_cwd_to_git_root() -> Result<()> {
	let project_root_dir = get_project_root_dir().ok_or("Not in a valid git repo")?;
	env::set_current_dir(project_root_dir).map_err(|_| "Something went wrong".into())
}

// TODO: eval if this should be used to get the root dir, maybe expose it from the git module
// How does this differ from open()?
// let repo = Repository::discover(".")?;
// Get the path to the .git directory
// let git_dir = repo.path();
// The parent of the .git directory is the root of the repository
// let root_dir = git_dir.parent().unwrap();

fn get_project_root_dir() -> Option<PathBuf> {
	let mut cwd = env::current_dir().ok()?;

	loop {
		let git_dir = cwd.join(".git");
		if git_dir.is_dir() {
			return Some(cwd);
		}

		if !cwd.pop() {
			break;
		}
	}

	None
}

mod args;
mod authors;
mod cli;
mod common;
mod error;
mod git;
mod handler;
