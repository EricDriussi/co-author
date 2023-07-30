use std::{
    io::{stdin, stdout},
    process,
};

use co_author::{cli::Cli, run_with_cli};

// TODO.Review &str vs String
fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let git_service = git::libgit_setup()?;
    let authors_service = authors::fs_setup();
    let cli = Cli::new(stdin().lock(), stdout().lock());
    return run_with_cli(git_service, authors_service, cli);
}
