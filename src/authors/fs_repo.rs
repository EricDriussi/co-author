use std::{
	env,
	error::Error,
	fs::File,
	io::{BufRead, BufReader, Lines},
	path::PathBuf,
	result::Result,
};

use super::{
	author::{Author, AuthorsRepo},
	author_err::AuthorError,
};

pub struct FSRepo {
	src: PathBuf,
}

impl FSRepo {
	pub fn default(default_authors_file: String) -> Result<Self, Box<dyn Error>> {
		let default_file = PathBuf::from(default_authors_file);
		match default_file.is_file() {
			true => Ok(Self { src: default_file }),
			false => Self::try_with_local_file(),
		}
	}

	pub fn from(authors_file: String) -> Result<Self, Box<dyn Error>> {
		let path = PathBuf::from(authors_file);
		match path.is_file() {
			true => Ok(Self { src: path }),
			false => Err(AuthorError::with(format!(
				"No file at path {:?}",
				path.to_str().unwrap()
			))),
		}
	}

	fn try_with_local_file() -> Result<FSRepo, Box<dyn Error>> {
		let mut local_file = env::current_dir().unwrap();
		local_file.push("authors");
		match local_file.is_file() {
			true => Ok(Self { src: local_file }),
			false => Err(AuthorError::with("No file found!".to_string())),
		}
	}

	fn read_lines(&self) -> Option<Lines<BufReader<File>>> {
		match File::open(&self.src) {
			Ok(file) => Some(BufReader::new(file).lines()),
			Err(_) => None,
		}
	}

	fn filter_by_alias(line: &str, aliases: &[String]) -> bool {
		aliases.iter().any(|given_alias| {
			let found_alias: &str = line.split(',').collect::<Vec<&str>>()[0];
			given_alias.eq_ignore_ascii_case(found_alias.trim())
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

	fn add_matching_signature(alias: String, valid_lines: &[String], matching_authors: &mut Vec<Author>) {
		for line in valid_lines {
			if Self::filter_by_alias(line, &[alias.clone()]) {
				if let Some(author) = Self::parse_author(line) {
					matching_authors.push(author);
				}
			}
		}
	}

	fn extract_from_lines(lines: Lines<BufReader<File>>, aliases: Vec<String>, matching_authors: &mut Vec<Author>) {
		let valid_lines = lines.map_while(Result::ok).collect::<Vec<_>>();
		for alias in aliases {
			Self::add_matching_signature(alias, &valid_lines, matching_authors);
		}
	}
}

impl AuthorsRepo for FSRepo {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		let mut matching_authors: Vec<Author> = Vec::new();

		if let Some(lines) = self.read_lines() {
			Self::extract_from_lines(lines, aliases, &mut matching_authors)
		};

		matching_authors
	}

	fn all(&self) -> Vec<Author> {
		match self.read_lines() {
			Some(lines) => lines
				.map_while(Result::ok)
				.filter_map(|line| Self::parse_author(line.as_str()))
				.collect(),
			None => Vec::new(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_read_lines() {
		let repo = FSRepo::from("src/authors/test/data/authors".to_string()).unwrap();
		let contents = repo.read_lines();

		assert!(contents.is_some());
	}

	#[test]
	fn should_filter_by_alias() {
		let matching_alias = FSRepo::filter_by_alias("a,John,Doe", &[String::from("a")]);
		assert!(matching_alias);

		let no_matching_alias = FSRepo::filter_by_alias("b,Jane,Dane", &[String::from("a")]);
		assert!(!no_matching_alias);
	}

	#[test]
	fn should_parse_author() {
		let valid_result = FSRepo::parse_author("a,John,Doe");
		assert_eq!(valid_result, Some(Author::new("a", "John", "Doe")));

		let invalid_result = FSRepo::parse_author("hi,invalid_line");
		assert_eq!(invalid_result, None);
	}
}
