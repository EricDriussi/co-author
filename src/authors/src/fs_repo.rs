use std::{env, fs::File, io::{BufRead, BufReader, Lines, Result}, path::PathBuf};

use crate::author::{Author, AuthorsRepo};

pub struct FSRepo {
    src: PathBuf,
}

impl FSRepo {
    pub fn new(src: PathBuf) -> Self {
        Self { src }
    }

    pub fn from(authors_file: Option<String>) -> std::result::Result<Self, String> {
        if authors_file.is_some() {
            let path = PathBuf::from(authors_file.unwrap());
            if path.is_file() {
                Ok(Self { src: path })
            } else {
                Err(format!("No file found at path {:?}", path.to_str()))
            }
        } else if authors_file.is_none() {
            let home_dir = env::var("HOME").unwrap();
            let path = PathBuf::from(format!("{}/.config/coa/authors", home_dir));
            if path.is_file() {
                Ok(Self { src: path })
            } else {
                let mut path = env::current_dir().unwrap();
				path.push("authors");
                if path.is_file() {
                    Ok(Self { src: path })
                } else {
                    Err("No authors file found!".to_string())
                }
            }
        } else {
            Err("FileSystem error!".to_string())
        }
    }

    fn read_lines(&self) -> Result<Lines<BufReader<File>>> {
        let file = File::open(&self.src)?;
        Ok(BufReader::new(file).lines())
    }

    fn filter_by_alias(line: &str, aliases: &[String]) -> bool {
        aliases.iter().any(|given_alias| {
            let found_alias: &str = line.split(',').collect::<Vec<&str>>()[0];
            return given_alias.eq_ignore_ascii_case(found_alias.trim());
        })
    }

    fn parse_author(line: &str) -> Option<Author> {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() == 3 {
            Some(Author::new(fields[0], fields[1], fields[2]))
        } else {
            None
        }
    }
}

impl AuthorsRepo for FSRepo {
    fn find(&self, aliases: Vec<String>) -> Vec<Author> {
        match self.read_lines() {
            Ok(lines) => lines
                .filter_map(Result::ok)
                .filter(|line| Self::filter_by_alias(line, &aliases))
                .filter_map(|matching_line| Self::parse_author(matching_line.as_str()))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn all(&self) -> Vec<Author> {
        match self.read_lines() {
            Ok(lines) => lines
                .filter_map(Result::ok)
                .filter_map(|line| Self::parse_author(line.as_str()))
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_lines() {
        let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();
        let contents = repo.read_lines();

        assert!(contents.is_ok());
    }

    #[test]
    fn should_filter_by_alias() {
        let matching_alias = FSRepo::filter_by_alias("a,John,Doe", &[String::from("a")]);
        assert_eq!(matching_alias, true);

        let no_matching_alias = FSRepo::filter_by_alias("b,Jane,Dane", &[String::from("a")]);
        assert_eq!(no_matching_alias, false);
    }

    #[test]
    fn should_parse_author() {
        let valid_result = FSRepo::parse_author("a,John,Doe");
        assert_eq!(valid_result, Some(Author::new("a", "John", "Doe")));

        let invalid_result = FSRepo::parse_author("hi,invalid_line");
        assert_eq!(invalid_result, None);
    }
}
