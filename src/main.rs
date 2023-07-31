use clap::Parser;
use std::{
    io::{stdin, stdout},
    process,
};

use co_author::{cli::Cli, run_interactive};

/// Co-author your git commits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File containing a csv list of authors (alias,name,email)
    #[arg(short, long)]
    file: Option<String>,
}

// TODO: modify --path -> --file
// TODO: pass list of aliases as arg (--list)
// TODO: option to add all aliases in file (--all), don't ask for aliases
// TODO: option to open commit buffer instead of asking for commit message (--editor), pre-populated with co-authors OR...
// TODO: add dedicated flag for commit message (--message), else ☝️
// TODO: option to pre-populate with last commit message (--pre-populate), for both -m and -e
// TODO: make current default cli prompting a specific use case/flag (--interactive)
// TODO: running co-authors on its own should print the help menu (let matches = App::new("myprog").setting(AppSettings::ArgRequiredElseHelp).get_matches();)
// TODO: sort authors by name when printing
// TODO: automatically create aliases for authors
// TODO: look at config-rs for config/paths handling
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
    let authors_service = authors::fs_setup(args.file)?;

    let cli = Cli::new(stdin().lock(), stdout().lock());
    return run_interactive(git_service, authors_service, cli);
}
