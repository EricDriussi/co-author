use args::Args;
use clap::Parser;
use error::Error;
use orchestrator::Orchestrator;

// TODO: automatically create on the fly aliases for authors
// TODO: add amend option -> update authors if given and update message if given (how does this work with --editor?)
// TODO: use with fzf or add fuzzy finding

fn main() {
	let args = Args::parse();
	if let Err(e) = run(args) {
		eprintln!("[Error] {e}");
		std::process::exit(1);
	}
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn run(args: Args) -> Result<()> {
	let cli = ui::di::init()?;
	let service = git::di::init()?;
	let provider = authors::di::init(&args.file)?;
	Orchestrator::exec(args, cli, service, provider)
}

mod args;
mod authors;
mod common;
mod error;
mod git;
mod orchestrator;
mod ui;
