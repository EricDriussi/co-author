use std::{
	env,
	error::Error,
	fs::File,
	io::{BufRead, BufReader, Lines},
	path::PathBuf,
	result::Result,
};

use crate::conf;

use super::{
	author::{Author, AuthorsRepo},
	author_err::AuthorError,
};

pub struct FSRepo {
	src: PathBuf,
}

impl FSRepo {
	pub fn from_cwd_with_home_fallback() -> Result<Self, Box<dyn Error>> {
		let mut local_file = env::current_dir().unwrap();
		local_file.push(conf::authors_file_name());
		if local_file.is_file() {
			return Ok(Self { src: local_file });
		}

		let default_file = PathBuf::from(conf::authors_file_path());
		if default_file.is_file() {
			return Ok(Self { src: default_file });
		}
		Err(AuthorError::with("No file found!".to_string()))
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

	fn read_lines(&self) -> Option<Lines<BufReader<File>>> {
		match File::open(&self.src) {
			Ok(file) => Some(BufReader::new(file).lines()),
			Err(_) => None,
		}
	}

	fn line_contains_any_alias(line: &str, aliases: &[String]) -> bool {
		aliases.iter().any(|given_alias| {
			let found_alias: &str = line.split(',').collect::<Vec<&str>>()[0];
			given_alias.eq_ignore_ascii_case(found_alias.trim())
		})
	}

	fn parse_author(line: &str) -> Option<Author> {
		let fields: Vec<&str> = line.split(',').collect();
		if fields.len() != 3 {
			return None;
		}
		Some(Author::from(fields[0], fields[1], fields[2]))
	}

	fn extract_author(line: &str, aliases: &[String]) -> Option<Author> {
		if Self::line_contains_any_alias(line, aliases) {
			Self::parse_author(line)
		} else {
			None
		}
	}

	fn extract_mathcing_authors_from_lines(alias: String, valid_lines: &[String]) -> Vec<Author> {
		let mut matching_authors: Vec<Author> = Vec::new();
		valid_lines.iter().for_each(|line| {
			if let Some(author) = Self::extract_author(line, &[alias.clone()]) {
				matching_authors.push(author);
			}
		});
		matching_authors
	}
}

impl AuthorsRepo for FSRepo {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		match self.read_lines() {
			None => Vec::new(),
			Some(lines) => {
				let valid_lines = lines.map_while(Result::ok).collect::<Vec<_>>();
				let mut matching_authors: Vec<Author> = Vec::new();
				aliases.iter().for_each(|alias| {
					matching_authors.append(&mut Self::extract_mathcing_authors_from_lines(
						alias.clone(),
						&valid_lines,
					));
				});
				matching_authors
			}
		}
	}

	fn all(&self) -> Vec<Author> {
		match self.read_lines() {
			None => Vec::new(),
			Some(lines) => lines
				.map_while(Result::ok)
				.filter_map(|line| Self::parse_author(line.as_str()))
				.collect(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_read_lines() {
		let repo = FSRepo::from(conf::dummy_data()).unwrap();
		let contents = repo.read_lines();

		assert!(contents.is_some());
	}

	#[test]
	fn should_filter_by_alias() {
		let matching_alias = FSRepo::line_contains_any_alias("a,John,Doe", &[String::from("a")]);
		assert!(matching_alias);

		let no_matching_alias = !FSRepo::line_contains_any_alias("b,Jane,Dane", &[String::from("a")]);
		assert!(no_matching_alias);
	}

	#[test]
	fn should_parse_author() {
		let valid_result = FSRepo::parse_author("j,John,email");
		assert!(valid_result.is_some_and(|a| a == Author::from("j", "John", "email")));

		let invalid_result = FSRepo::parse_author("hi,invalid_line");
		assert!(invalid_result.is_none());
	}

	#[test]
	fn should_extract_author() {
		let valid_result = FSRepo::extract_author("j,John,email", &[String::from("j")]);
		assert!(valid_result.is_some_and(|a| a == Author::from("j", "John", "email")));

		let invalid_result = FSRepo::extract_author("a,alice,gmail", &[String::from("j")]);
		assert!(invalid_result.is_none());
	}

	#[test]
	fn should_extract_mathcing_authors_from_lines() {
		let matching_authors: Vec<Author> = FSRepo::extract_mathcing_authors_from_lines(
			"j".to_string(),
			&["j,John,email".to_string(), "a,alice,gmail".to_string()],
		);

		assert!(matching_authors.contains(&Author::from("j", "John", "email")));
		assert!(!matching_authors.contains(&Author::from("a", "alice", "gmail")));
	}
}
