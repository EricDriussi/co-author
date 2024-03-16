use std::path::PathBuf;

use crate::authors::csv::provider::CSVReader;
use crate::authors::err::AuthorsError;
use crate::authors::load_mode::LoadMode;
use crate::common::conf;
use crate::common::fs::test::util::dummy_file::DummyFile;
use crate::error::{assert_error_contains_msg, assert_error_type};
use crate::Result;
use crate::{authors::author::AuthorsProvider, common::fs::wrapper::MockFileLoader};
use mockall::predicate::{self, eq};
use mockall::Sequence;

const IRRELEVANT_FILE_PATH: &str = "a/path/file.hi";

#[test]
fn build_from_given_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_if_present()
		.with(eq(IRRELEVANT_FILE_PATH.to_string()))
		.returning(|_| Some(Box::new(DummyFile::empty())));

	assert!(load_from_path(&mock_file_loader).is_ok());
}

#[test]
fn not_build_from_given_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_if_present()
		.with(eq(IRRELEVANT_FILE_PATH.to_string()))
		.returning(|_| None);

	let result = load_from_path(&mock_file_loader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, IRRELEVANT_FILE_PATH);
}

#[test]
fn build_using_fallback() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_if_present()
		.returning(|_| Some(Box::new(DummyFile::empty())));

	assert!(load_from_cwd(&mock_file_loader).is_ok());
}

#[test]
fn not_build_using_fallback() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_if_present().returning(|_| None);

	let result = load_from_cwd(&mock_file_loader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, "$PWD or $HOME");
}

#[test]
fn fallback_sensibly() {
	let xdg_config = "a_path";
	let home = "a_home";
	let cwd = "/tmp";
	std::env::set_current_dir(PathBuf::from(cwd)).expect("Could not set current dir for tests");
	std::env::set_var("XDG_CONFIG_HOME", xdg_config);
	std::env::set_var("HOME", home);
	let file_path = &conf::authors_file();
	let dir_path = &conf::authors_dir();
	let mut seq = Sequence::new();
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_if_present()
		.with(eq(format!("{cwd}/{file_path}")))
		.returning(|_| None)
		.times(1)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_load_if_present()
		.with(eq(format!("{xdg_config}/{dir_path}/{file_path}")))
		.returning(|_| None)
		.times(1)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_load_if_present()
		.with(eq(format!("{home}/.config/{dir_path}/{file_path}")))
		.times(1)
		.returning(|_| None)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_load_if_present()
		.with(eq(format!("{home}/.{dir_path}/{file_path}")))
		.times(1)
		.returning(|_| None)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_load_if_present()
		.with(eq(format!("{home}/{file_path}")))
		.times(1)
		.returning(|_| None)
		.in_sequence(&mut seq);

	let result = load_from_cwd(&mock_file_loader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, "$PWD or $HOME");
}

#[test]
fn provide_all_authors_in_file() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_if_present().times(1).returning(|_| {
		Some(Box::new(DummyFile::with(vec![
			"a,Name Surname,someone@users.noreply.github.com",
			"b,username,something@gmail.com",
		])))
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.all();

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn provide_only_author_matching_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_if_present().times(1).returning(|_| {
		Some(Box::new(DummyFile::with(vec![
			"a,Name Surname,someone@users.noreply.github.com",
			"b,username,something@gmail.com",
		])))
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["a".to_string()]);

	assert_eq!(retrieved_authors.len(), 1);
}

#[test]
fn provide_all_authors_matching_an_alias() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_if_present().times(1).returning(|_| {
		Some(Box::new(DummyFile::with(vec![
			"a,Name Surname,someone@users.noreply.github.com",
			"b,username,something@gmail.com",
			"b,username2,something2@gmail.com",
		])))
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["b".to_string()]);

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn provide_no_author_when_alias_doesnt_match() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_if_present()
		.with(predicate::always())
		.times(1)
		.returning(|_| {
			Some(Box::new(DummyFile::with(vec![
				"a,Name Surname,someone@users.noreply.github.com",
			])))
		});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["z".to_string()]);

	assert_eq!(retrieved_authors.len(), 0);
}

fn load_from_path(file_loader: &MockFileLoader) -> Result<CSVReader> {
	CSVReader::load(&LoadMode::FromPath {
		file_loader,
		path: IRRELEVANT_FILE_PATH,
	})
}

fn load_from_cwd(file_loader: &MockFileLoader) -> Result<CSVReader> {
	CSVReader::load(&LoadMode::FromCwd { file_loader })
}
