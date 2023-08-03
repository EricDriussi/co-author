use std::{
	io::{stdin, stdout},
	process,
};

use clap::Parser;

use co_author::{cli::Cli, run_interactive, run_interactive_no_ask_aliases};

/// Co-author your git commits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// File containing a csv list of authors (alias,name,email)
	#[arg(short, long)]
	file: Option<String>,

	/// List of comma spearated aliases
	#[arg(short, long)]
	list: Option<String>,
}

// TODO: option to add all aliases in file (--all), don't ask for aliases
// TODO: option to open commit buffer instead of asking for commit message (--editor), pre-populated with co-authors OR...
// TODO: add dedicated flag for commit message (--message), else ☝️
// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and -e
// TODO: make current default cli prompting a specific use case/flag (--interactive)
// TODO: running co-authors on its own should print the help menu (let matches = App::new("myprog").setting(AppSettings::ArgRequiredElseHelp).get_matches();)
// TODO: sort authors by name when printing
// TODO: automatically create aliases for authors
// TODO: use with fzf or add fuzzy finding

fn main() {
	let args = Args::parse();
	match run(args) {
		Ok(_) => (),
		Err(e) => {
			eprintln!("[Error] {}", e);
			process::exit(1);
		}
	}
}

fn run(args: Args) -> Result<(), String> {
	let git_service = git::libgit_setup()?;
	let authors_service = match args.file {
		Some(file) => authors::fs_setup_from_file(file)?,
		None => authors::fs_default_setup()?,
	};

	let cli = Cli::new(stdin().lock(), stdout().lock());

	if args.list.is_some() {
		return run_interactive_no_ask_aliases(git_service, authors_service, cli, args.list.unwrap());
	}
	return run_interactive(git_service, authors_service, cli);
}
