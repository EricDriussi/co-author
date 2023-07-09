use std::io::{self, BufRead};

pub fn prompt_user(prompt: &str, input: Option<&mut dyn BufRead>) -> String {
    println!("{}", prompt);

    let mut user_input = String::new();
    input
        .unwrap_or(&mut default_to_stdin())
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    return user_input.trim().to_string();
}

fn default_to_stdin() -> io::StdinLock<'static> {
    io::stdin().lock()
}

#[cfg(test)]
mod test;
