use args::Args;
use clap::Parser;
use orch::Orchestrator;
use std::{error::Error, process, result};

// TODO: improve tests
// TODO: review optional/result handling
// TODO: fix bug with first commit in new repo
// TODO: automatically create on the fly aliases for authors
// TODO: add amend option -> update authors if given and update message if given (how does this work with --editor?)
// TODO: use with fzf or add fuzzy finding

fn main() {
	let args = Args::parse();
	if let Err(e) = run(args) {
		eprintln!("[Error] {e}");
		process::exit(1);
	}
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;
// TODO: use custom error once git module and handler.rs are refactored
// pub type Result<T> = result::Result<T, Error>;

fn run(args: Args) -> Result<()> {
	let mut orchestrator = Orchestrator::new(args, ui::di::init()?)?;
	let authors_signatures = orchestrator.handle_authors()?;
	orchestrator.handle_commit_msg(authors_signatures)
}

mod args;
mod authors;
mod common;
mod error;
mod git;
mod orch;
mod ui;
