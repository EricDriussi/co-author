use crate::authors::author::{Author, AuthorsProvider};
use crate::conf;
use crate::fs::{MockFileLoader, Readable};

use mockall::predicate::{self, eq};

use super::provider::CSVReader;

#[test]
fn should_build_from_file_in_cwd() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_file()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from_cwd_fallback_home(&mock_file_loader).is_ok());
}

#[test]
fn should_fallback_to_home_file_when_no_file_in_cwd() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_file()))
		.times(1)
		.returning(|_| None);
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_path()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from_cwd_fallback_home(&mock_file_loader).is_ok());
}

#[test]
fn should_error_when_file_is_not_in_cwd_nor_home() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_file()))
		.times(1)
		.returning(|_| None);
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_path()))
		.times(1)
		.returning(|_| None);

	let result = CSVReader::from_cwd_fallback_home(&mock_file_loader);
	assert!(matches!(result, Err(e) if e.to_string().contains("No file found in cwd or home")));
}

#[test]
fn should_build_from_given_file() {
	let irrelevant_path = "a/path";
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(irrelevant_path.to_string()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from(&mock_file_loader, irrelevant_path).is_ok());
}

#[test]
fn should_not_build_from_given_file() {
	let irrelevant_path = "a/path";
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(irrelevant_path.to_string()))
		.times(1)
		.returning(|_| None);

	let result = CSVReader::from(&mock_file_loader, irrelevant_path);
	assert!(matches!(result, Err(e) if e.to_string().contains("No file at path")));
}

#[test]
fn should_provide_all_authors_in_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = repo.all();

	assert_eq!(retrieved_authors.len(), 4);
	assert!(retrieved_authors.contains(&Author::from("a", "Name Surname", "someone@users.noreply.github.com")));
	assert!(retrieved_authors.contains(&Author::from("b", "username", "something@gmail.com")));
	assert!(retrieved_authors.contains(&Author::from("b", "username2", "something2@gmail.com")));
	assert!(retrieved_authors.contains(&Author::from("ab", "Another Surname", "someone@something.hi")));
}

#[test]
fn should_provide_author_given_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let alias = "a";
	let actual_author = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_author,
		[Author::from(alias, "Name Surname", "someone@users.noreply.github.com")]
	);
}

#[test]
fn should_provide_all_authors_given_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let alias = "b";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_authors,
		[
			Author::from(alias, "username", "something@gmail.com"),
			Author::from(alias, "username2", "something2@gmail.com"),
		]
	);
}

#[test]
fn should_provide_no_author_when_no_matching_alias_are_found() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let alias = "z";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(actual_authors, []);
}

pub struct DummyAuthorsFile {
	content: Vec<String>,
}

impl DummyAuthorsFile {
	// TODO: this should take the vec as param
	pub fn default_content() -> Self {
		Self {
			content: (vec![
				"a,Name Surname,someone@users.noreply.github.com",
				"b,username,something@gmail.com",
				"b,username2,something2@gmail.com",
				"ab,Another Surname,someone@something.hi",
			])
			.into_iter()
			.map(String::from)
			.collect(),
		}
	}
}

impl Readable for DummyAuthorsFile {
	fn non_empty_lines(&self) -> Vec<String> {
		self.content.clone()
	}

	fn all_lines(&self) -> Vec<String> {
		self.content.clone()
	}
}
