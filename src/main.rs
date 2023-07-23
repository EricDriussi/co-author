use std::{env, error::Error, process};

use authors::{app_service::AuthService, fs_repo::FSRepo};
use git::{app_service::GitService, libgit_repo::LibGitRepo};

mod cli;
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

    let app_service = setup_authors_service();
    app_service.print_available();

    let aliases = cli::ask_for_aliases(None);
    let found_authors = app_service.find_authors(aliases);

    let commit_body = cli::ask_for_commit_message(None)?;

    git_service.commit(commit_body.as_str(), found_authors)?;
    Ok(())
}

fn setup_authors_service() -> AuthService<FSRepo> {
    let home_dir = env::var("HOME").unwrap();
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
