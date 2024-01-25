use crate::authors::author::{Author, AuthorsProvider};
use crate::fs_wrapper::{File, MockFsWrapper};

use mockall::predicate::{self, eq};

use super::provider::CSVReader;

#[test]
fn should_connect_to_an_authors_file_in_cwd_if_available() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).is_ok());
}

#[test]
fn should_connect_to_the_default_authors_file_if_no_file_is_available_in_cwd() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| None);
	mock_fs_wrapper
		.expect_file_in_abs_path()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).is_ok());
}

#[test]
fn should_error_when_neither_cwd_or_default_authors_file_are_available() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| None);
	mock_fs_wrapper
		.expect_file_in_abs_path()
		.with(predicate::always())
		.times(1)
		.returning(|_| None);

	let result = CSVReader::from_cwd_fallback_home(&mock_fs_wrapper);
	assert!(matches!(result, Err(e) if e.to_string().contains("No file found")));
}

#[test]
fn should_connect_to_a_given_existing_authors_file() {
	let an_authors_file_path = "/tmp/an_authors_file";
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_abs_path()
		.with(eq(an_authors_file_path.to_string()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));

	assert!(CSVReader::from(&mock_fs_wrapper, an_authors_file_path).is_ok());
}

#[test]
fn should_not_connect_to_a_given_non_existing_file() {
	let an_authors_file_path = "/tmp/an_authors_file";
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_abs_path()
		.with(eq(an_authors_file_path.to_string()))
		.times(1)
		.returning(|_| None);

	let result = CSVReader::from(&mock_fs_wrapper, an_authors_file_path);
	assert!(matches!(result, Err(e) if e.to_string().contains("No file at path")));
}

#[test]
fn should_return_all_authors_from_file() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).expect("Could not setup FSProvider for test");

	let retrieved_authors = repo.all();

	assert_eq!(retrieved_authors.len(), 4);
	assert!(retrieved_authors.contains(&Author::from("a", "Name Surname", "someone@users.noreply.github.com")));
	assert!(retrieved_authors.contains(&Author::from("b", "username", "something@gmail.com")));
	assert!(retrieved_authors.contains(&Author::from("b", "username2", "something2@gmail.com")));
	assert!(retrieved_authors.contains(&Author::from("ab", "Another Surname", "someone@something.hi")));
}

#[test]
fn should_fetch_authors_based_on_alias() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).expect("Could not setup FSProvider for test");

	let alias = "a";
	let actual_author = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_author,
		[Author::from(alias, "Name Surname", "someone@users.noreply.github.com")]
	);
}

#[test]
fn should_fetch_all_authors_for_a_given_alias() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).expect("Could not setup FSProvider for test");

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
fn should_return_an_empty_list_if_no_author_matches_alias() {
	let mut mock_fs_wrapper = MockFsWrapper::new();
	mock_fs_wrapper
		.expect_file_in_cwd()
		.with(predicate::always())
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::default_content())));
	let repo = CSVReader::from_cwd_fallback_home(&mock_fs_wrapper).expect("Could not setup FSProvider for test");

	let alias = "z";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(actual_authors, []);
}

pub struct DummyAuthorsFile {
	content: Vec<String>,
}

impl DummyAuthorsFile {
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

impl File for DummyAuthorsFile {
	fn read_lines(&self) -> Vec<String> {
		self.content.clone()
	}
}
