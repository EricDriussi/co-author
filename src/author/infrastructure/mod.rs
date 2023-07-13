use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::domain::{Author, Repository};

pub struct FSRepo {
    src: PathBuf,
}

impl FSRepo {
    fn new(src: &str) -> Self {
        Self {
            src: PathBuf::from(src),
        }
    }

    fn read_file(&self) -> Vec<String> {
        match self.src.to_str() {
            Some(path) => {
                match File::open(path) {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
                        return lines;
                    }
                    Err(_) => return Vec::new(),
                };
            }
            None => return Vec::new(),
        };
    }
}

impl Repository for FSRepo {
    fn find_authors(&self, aliases: Vec<&str>) -> Vec<Author> {
        match self.src.to_str() {
            Some(path) => {
                match File::open(path) {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        let authors: Vec<Author> = reader
                            .lines()
                            .filter_map(|line| line.ok())
                            .filter(|line| aliases.iter().any(|prefix| line.starts_with(*prefix)))
                            .filter_map(|line| {
                                let parts: Vec<&str> = line.split(',').collect();
                                if parts.len() == 3 {
                                    Some(Author::new(parts[0], parts[1], parts[2]))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        return authors;
                    }
                    Err(_) => return Vec::new(),
                };
            }
            None => return Vec::new(),
        };
    }

    fn all_authors(&self) -> Vec<Author> {
        let list = self.read_file();

        let mut array: Vec<Author> = Vec::new();

        for line in list {
            let fields: Vec<&str> = line.split(',').collect();

            let person = Author::new(fields[0], fields[1], fields[2]);
            array.push(person);
        }

        return array;
    }
}

#[cfg(test)]
mod test;
