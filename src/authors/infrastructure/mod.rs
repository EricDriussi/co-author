use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::PathBuf,
};

use super::domain::{Author, Repository};

pub struct FSRepo {
    src: PathBuf,
}

impl FSRepo {
    pub fn new(src: &str) -> Self {
        Self {
            src: PathBuf::from(src),
        }
    }

    fn read_lines(&self) -> Result<Lines<BufReader<File>>> {
        let file = File::open(&self.src)?;
        Ok(BufReader::new(file).lines())
    }

    fn filter_by_alias(&self, line: &str, aliases: &[String]) -> bool {
        aliases.iter().any(|alias| line.starts_with(alias))
    }

    fn parse_author(&self, line: &str) -> Option<Author> {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() == 3 {
            Some(Author::new(fields[0], fields[1], fields[2]))
        } else {
            None
        }
    }
}

impl Repository for FSRepo {
    fn find_authors(&self, aliases: Vec<String>) -> Vec<Author> {
        match self.read_lines() {
            Ok(lines) => lines
                .filter_map(Result::ok)
                .filter(|line| self.filter_by_alias(line, &aliases))
                .filter_map(|matching_line| self.parse_author(matching_line.as_str()))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn all_authors(&self) -> Vec<Author> {
        match self.read_lines() {
            Ok(lines) => lines
                .filter_map(Result::ok)
                .filter_map(|line| self.parse_author(line.as_str()))
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod test;
