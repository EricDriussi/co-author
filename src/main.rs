use std::{env, error::Error};

use co_author::authors::{application::Service, infrastructure::FSRepo};

mod cli;
fn main() -> Result<(), Box<dyn Error>> {
    let home_dir = env::var("HOME").unwrap();
    let file_path = format!("{}/.config/coa/authors", home_dir);

    let repo = FSRepo::new(file_path.as_str());
    let app_service = Service::new(repo);

    app_service.print_available();

    let aliases = cli::ask_for_aliases(None);
    for alias in aliases {
        println!("Alias: {}", alias);
    }
    let commit_body = cli::ask_for_commit_message(None)?;
    println!("Commit body: {}", commit_body);
    Ok(())
}
