use std::{
    env,
    error::Error,
    io::{stdin, stdout},
    process,
};

use authors::{app_service::AuthService, fs_repo::FSRepo};
use co_author::{cli::Cli, run_with_cli};
use git::{app_service::GitService, libgit_repo::LibGitRepo};

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let git_service = setup_git_service()?;
    let authors_service = setup_authors_service();
    let cli = Cli::new(stdin().lock(), stdout().lock());
    return run_with_cli(git_service, authors_service, cli);
}

fn setup_authors_service() -> AuthService<FSRepo> {
    let home_dir = env::var("HOME").unwrap();
    // TODO.handle author file location by param
    let file_path = format!("{}/.config/coa/authors", home_dir);

    let repo = FSRepo::new(file_path.as_str());
    return AuthService::new(repo);
}

fn setup_git_service() -> Result<GitService<LibGitRepo>, String> {
    let repo = LibGitRepo::new(env::current_dir().unwrap());
    let serv = match repo.open_if_valid() {
        Some(repo) => Ok(GitService::new(repo)),
        None => {
            return Err("Not a valid git repository".to_string());
        }
    };
    return serv;
}
