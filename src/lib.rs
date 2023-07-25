use std::{
    error::Error,
    io::{BufRead, Write},
};

use authors::{app_service::AuthService, author::AuthorRepo};
use cli::Cli;
use git::{app_service::GitService, git::GitRepo};

pub mod cli;

pub fn run_with_cli<T: GitRepo, Y: AuthorRepo, R: BufRead, W: Write>(
    git_service: GitService<T>,
    auth_service: AuthService<Y>,
    mut cli: Cli<R, W>,
) -> Result<(), Box<dyn Error>> {
    auth_service.print_available();
    let aliases = cli.ask_for_aliases();
    let found_authors = auth_service.find_authors(aliases);
    let commit_body = cli.ask_for_commit_message()?;

    git_service.commit(commit_body.as_str(), found_authors)?;
    Ok(())
    //TODO. return git_service.commit(commit_body.as_str(), found_authors);
}
