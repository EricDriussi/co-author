use std::{env, error::Error, process};

use co_author::authors::{application::Service, infrastructure::FSRepo};

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
    let app_service = setup_authors_service();
    app_service.print_available();

    let aliases = cli::ask_for_aliases(None);
    let found_authors = app_service.find_authors(aliases);
    for author in found_authors {
        println!("Alias: {}", author);
    }

    let commit_body = cli::ask_for_commit_message(None)?;
    println!("Commit body: {}", commit_body);
    Ok(())
}

fn setup_authors_service() -> Service<FSRepo> {
    let home_dir = env::var("HOME").unwrap();
    let file_path = format!("{}/.config/coa/authors", home_dir);

    let repo = FSRepo::new(file_path.as_str());
    return Service::new(repo);
}
