use std::io::{BufRead, Write};

mod reader;

pub struct Cli<R: BufRead, W: Write> {
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Cli<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Cli { input, output }
    }

    pub fn ask_for_commit_message(&mut self) -> Result<String, &'static str> {
        let commit_message = reader::prompt(
            "Enter your commit message:",
            &mut self.input,
            &mut self.output,
        );

        if commit_message.is_empty() {
            return Err("Commit message cannot be empty.");
        }
        return Ok(commit_message);
    }

    pub fn ask_for_aliases(&mut self) -> Vec<String> {
        let aliases = reader::prompt(
            "Enter co-authors aliases separated by spaces:",
            &mut self.input,
            &mut self.output,
        );

        return aliases.split_whitespace().map(|s| s.to_string()).collect();
    }
}

#[cfg(test)]
mod test;
