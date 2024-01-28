use crate::authors::author::AuthorsProvider;
use crate::authors::csv::provider::CSVReader;
use crate::conf;
use crate::fs::file::Readable;
use crate::fs::wrapper::MockFileLoader;

use mockall::predicate::{self, eq};

const IRRELEVANT_FILE_PATH: &str = "a/path/file.hi";

#[test]
fn should_build_from_file_in_cwd() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(conf::authors_csv_file()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::empty())));

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
		.returning(|_| Some(Box::new(DummyAuthorsFile::empty())));

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

	assert!(matches!(
	CSVReader::from_cwd_fallback_home(&mock_file_loader),
	Err(e) if e.to_string().contains("No file at $PWD or $HOME")));
}

#[test]
fn should_build_from_given_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(IRRELEVANT_FILE_PATH.to_string()))
		.times(1)
		.returning(|_| Some(Box::new(DummyAuthorsFile::empty())));

	assert!(CSVReader::from(&mock_file_loader, IRRELEVANT_FILE_PATH).is_ok());
}

#[test]
fn should_not_build_from_given_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(eq(IRRELEVANT_FILE_PATH.to_string()))
		.times(1)
		.returning(|_| None);

	assert!(matches!(
	CSVReader::from(&mock_file_loader, IRRELEVANT_FILE_PATH),
	Err(e) if e.to_string().contains(format!("No file at {IRRELEVANT_FILE_PATH}").as_str())));
}

#[test]
fn should_provide_all_authors_in_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| {
			Some(Box::new(DummyAuthorsFile::with(vec![
				"a,Name Surname,someone@users.noreply.github.com",
				"b,username,something@gmail.com",
			])))
		});
	let provider = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.all();

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn should_provide_only_author_matching_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| {
			Some(Box::new(DummyAuthorsFile::with(vec![
				"a,Name Surname,someone@users.noreply.github.com",
				"b,username,something@gmail.com",
			])))
		});
	let provider = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["a".to_string()]);

	assert_eq!(retrieved_authors.len(), 1);
}

#[test]
fn should_provide_all_authors_matching_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| {
			Some(Box::new(DummyAuthorsFile::with(vec![
				"a,Name Surname,someone@users.noreply.github.com",
				"b,username,something@gmail.com",
				"b,username2,something2@gmail.com",
			])))
		});
	let provider = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["b".to_string()]);

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn should_provide_no_author_when_alias_doesnt_match() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_file()
		.with(predicate::always())
		.times(1)
		.returning(|_| {
			Some(Box::new(DummyAuthorsFile::with(vec![
				"a,Name Surname,someone@users.noreply.github.com",
			])))
		});
	let provider = CSVReader::from_cwd_fallback_home(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["z".to_string()]);

	assert_eq!(retrieved_authors.len(), 0);
}

pub struct DummyAuthorsFile {
	content: Vec<String>,
}

impl DummyAuthorsFile {
	pub fn empty() -> Self {
		Self { content: (vec![]) }
	}

	pub fn with(content: Vec<&str>) -> Self {
		Self {
			content: content.into_iter().map(String::from).collect(),
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
