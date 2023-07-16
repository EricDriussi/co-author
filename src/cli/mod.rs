use std::io::BufRead;

mod reader;

pub fn ask_for_commit_message(test_input: Option<&mut dyn BufRead>) -> Result<String, &str> {
    let commit_message = reader::prompt_user("Enter your commit message: ", test_input);

    if commit_message.is_empty() {
        Err("Commit message cannot be empty.")
    } else {
        Ok(commit_message)
    }
}

pub fn ask_for_aliases(test_input: Option<&mut dyn BufRead>) -> Vec<String> {
    let aliases = reader::prompt_user("Enter co-authors aliases separated by spaces:", test_input);

    return aliases.split_whitespace().map(|s| s.to_string()).collect();
}

#[cfg(test)]
mod test;
