use clap::Parser;
use std::{
    io::{stdin, stdout},
    process,
};

use co_author::{cli::Cli, run_with_cli};

/// Co-author your git commits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the authors file
    #[arg(short, long)]
    path: Option<String>,
}

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
    let authors_service = authors::fs_setup(args.path)?;

    let cli = Cli::new(stdin().lock(), stdout().lock());
    return run_with_cli(git_service, authors_service, cli);
}
