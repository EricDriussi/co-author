use std::io::{BufRead, Write};

pub fn prompt<R: BufRead, W: Write>(prompt: &str, mut reader: R, mut writer: W) -> String {
    write!(writer, "{} ", prompt).unwrap();
    writer.flush().unwrap();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    return input.trim().to_string();
}

#[cfg(test)]
mod test;
