use std::error::Error;

mod cli;
fn main() -> Result<(), Box<dyn Error>> {
    let aliases = cli::ask_for_aliases();
    for alias in aliases {
        println!("Alias: {}", alias);
    }
    let commit_body = cli::ask_for_commit_message(None)?;
    println!("Commit body: {}", commit_body);
    Ok(())
}
