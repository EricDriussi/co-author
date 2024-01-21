use std::{env, error::Error, path::PathBuf, process};

mod git;
use clap::Parser;
use co_author::{args::Args, cli::FancyCli, handle_authors, handle_commit_msg};

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

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
	set_cwd_to_git_root()?;

	let mut cli = FancyCli::new();
	let authors_signatures = handle_authors(args, &mut cli)?;

	// FIXME. Find a way to pass this to handle_commit_msg (clone/copy)
	let git_service = git::libgit_setup()?;
	let prev = git_service.last_commit_message();

	if args.editor {
		if args.pre_populate {
			return git_service.commit_with_pre_populated_editor(prev.as_str(), authors_signatures);
		}
		return git_service.commit_with_editor(authors_signatures);
	}
	let msg = handle_commit_msg(args, &mut cli, prev)?;

	return git_service.commit(msg.as_str(), authors_signatures);
}

fn set_cwd_to_git_root() -> Result<(), Box<dyn Error>> {
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
