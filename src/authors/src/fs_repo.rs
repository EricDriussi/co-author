use std::{
	env,
	fs::File,
	io::{BufRead, BufReader, Lines, Result},
	path::PathBuf,
};

use crate::author::{Author, AuthorsRepo};

pub struct FSRepo {
	src: PathBuf,
}

impl FSRepo {
	pub fn default(default_authors_file: String) -> std::result::Result<Self, String> {
		let default_file = PathBuf::from(default_authors_file);
		return match default_file.is_file() {
			true => Ok(Self { src: default_file }),
			false => Self::try_with_local_file(),
		};
	}

	pub fn from(authors_file: String) -> std::result::Result<Self, String> {
		let path = PathBuf::from(authors_file);
		return match path.is_file() {
			true => Ok(Self { src: path }),
			false => Err(format!("No file found at path {:?}", path.to_str().unwrap())),
		};
	}

	fn try_with_local_file() -> std::result::Result<FSRepo, String> {
		let mut local_file = env::current_dir().unwrap();
		local_file.push("authors");
		return match local_file.is_file() {
			true => Ok(Self { src: local_file }),
			false => Err("No authors file found!".to_string()),
		};
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
		let repo = FSRepo::from("tests/data/authors".to_string()).unwrap();
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
