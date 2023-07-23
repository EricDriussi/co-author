use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::PathBuf,
};

use crate::author::{Author, AuthorRepo};

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
        // FIXME. starts_with -> split by ',' and === first item
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

impl AuthorRepo for FSRepo {
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
mod test {
    use super::*;

    #[test]
    fn should_read_lines() {
        let repo = FSRepo::new("tests/data/authors");
        let contents = repo.read_lines();

        assert!(contents.is_ok());
    }

    #[test]
    fn should_filter_by_alias() {
        let fs_repo = FSRepo::new("no_file_needed");

        let matching_alias = fs_repo.filter_by_alias("a,John,Doe", &[String::from("a")]);
        assert_eq!(matching_alias, true);

        let no_matching_alias = fs_repo.filter_by_alias("b,Jane,Dane", &[String::from("a")]);
        assert_eq!(no_matching_alias, false);
    }

    #[test]
    fn should_parse_author() {
        let fs_repo = FSRepo::new("no_file_needed");

        let valid_result = fs_repo.parse_author("a,John,Doe");
        assert_eq!(valid_result, Some(Author::new("a", "John", "Doe")));

        let invalid_result = fs_repo.parse_author("hi,invalid_line");
        assert_eq!(invalid_result, None);
    }
}
