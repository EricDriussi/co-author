use std::{
	io::{stdin, stdout},
	process,
};

use authors::author::Author;
use clap::Parser;

use co_author::{cli::Cli, exec};

// FIXME.Extract to file
/// Co-author your git commits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// File containing a csv list of authors (alias,name,email)
	#[arg(short, long)]
	file: Option<String>,

	/// List of comma spearated author aliases
	#[arg(short, long)]
	list: Option<String>,

	/// Use all available authors
	#[arg(short, long, conflicts_with("list"), default_value = "false")]
	all: bool,

	/// Specify commit message
	#[arg(short, long)]
	message: Option<String>,

	/// Open default editor for commit message
	#[arg(short, long, default_value = "false")]
	editor: bool,
}

// TODO: -l and -a should work with -m
// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and default buffer opening
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
	let authors = get_authors_signatures(&args)?;
	let commit_body = get_commit_message(&args)?;
	return exec(commit_body, authors);
}

fn get_commit_message(args: &Args) -> Result<String, String> {
	let mut cli = Cli::new(stdin().lock(), stdout().lock());
	if let Some(message) = &args.message {
		return Ok(message.to_string());
	}
	if args.editor {
		return Ok(cli.get_commit_from_editor().unwrap());
	}
	let commit_body = cli.ask_for_commit_message()?;
	Ok(commit_body)
}

fn get_authors_signatures(args: &Args) -> Result<Vec<String>, String> {
	let authors_service = match &args.file {
		Some(file) => authors::fs_setup_from_file(file.to_string())?,
		None => authors::fs_default_setup()?,
	};

	if args.all {
		return Ok(authors_service.all_signatures());
	}
	if let Some(list) = &args.list {
		let given_aliases = list.split(',').map(|alias| alias.to_string()).collect();
		return Ok(authors_service.signatures_of(given_aliases));
	}

	let mut cli = Cli::new(stdin().lock(), stdout().lock());
	print(authors_service.all_available());
	let aliases = cli.ask_for_aliases();
	return Ok(authors_service.signatures_of(aliases));
}

fn print(authors: Vec<Author>) {
	println!();
	for author in &authors {
		println!("{}", author);
	}
	println!();
}
