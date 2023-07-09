use std::io::BufRead;

mod reader;

pub fn ask_for_commit_message(input: Option<&mut dyn BufRead>) -> Result<String, &str> {
    let commit_message = reader::prompt_user("Enter your commit message: ", input);

    if commit_message.is_empty() {
        Err("Commit message cannot be empty.")
    } else {
        Ok(commit_message)
    }
}

pub fn ask_for_aliases() -> Vec<&str> {
    let aliases = reader::prompt_user("Enter co-authors aliases separated by spaces:", None);

    aliases.split_whitespace().collect()
}

#[cfg(test)]
mod test;
