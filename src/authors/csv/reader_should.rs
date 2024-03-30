use super::reader::LoadMode;
use crate::authors::author::AuthorsProvider;
use crate::authors::csv::reader::CSVReader;
use crate::authors::err::AuthorsError;
use crate::common::conf;
use crate::common::fs::file_reader::MockReader;
use crate::error::{assert_error_contains_msg, assert_error_type};
use crate::Result;
use mockall::predicate::{self, eq};
use mockall::Sequence;
use std::path::PathBuf;

const IRRELEVANT_FILE_PATH: &str = "a/path/file.hi";

#[test]
fn build_from_given_file() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(IRRELEVANT_FILE_PATH)))
		.returning(|_| Ok(vec![]));

	assert!(load_from_path(&mock_file_loader).is_ok());
}

#[test]
fn not_build_from_given_file() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(IRRELEVANT_FILE_PATH)))
		.returning(|_| Err("oops".into()));

	let result = load_from_path(&mock_file_loader);

	assert_error_type(&result, &AuthorsError::NotFound(String::from(IRRELEVANT_FILE_PATH)));
	assert_error_contains_msg(&result, IRRELEVANT_FILE_PATH);
}

#[test]
fn build_using_fallback() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader.expect_read_non_empty_lines().returning(|_| Ok(vec![]));

	assert!(load_from_cwd(&mock_file_loader).is_ok());
}

#[test]
fn not_build_using_fallback() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader
		.expect_read_non_empty_lines()
		.returning(|_| Err("oops".into()));

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
	let mut mock_file_loader = MockReader::new();
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{cwd}/{file_path}"))))
		.returning(|_| Err("oops".into()))
		.times(1)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{xdg_config}/{dir_path}/{file_path}"))))
		.returning(|_| Err("oops".into()))
		.times(1)
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/.config/{dir_path}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/.{dir_path}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);

	let result = load_from_cwd(&mock_file_loader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, "$PWD or $HOME");
}

#[test]
fn provide_all_authors_in_file() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader.expect_read_non_empty_lines().times(1).returning(|_| {
		Ok(vec![
			"a,Name Surname,someone@users.noreply.github.com".to_string(),
			"b,username,something@gmail.com".to_string(),
		])
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.all();

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn provide_only_author_matching_an_alias() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader.expect_read_non_empty_lines().times(1).returning(|_| {
		Ok(vec![
			"a,Name Surname,someone@users.noreply.github.com".to_string(),
			"b,username,something@gmail.com".to_string(),
		])
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["a".to_string()]);

	assert_eq!(retrieved_authors.len(), 1);
}

#[test]
fn provide_all_authors_matching_an_alias() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader.expect_read_non_empty_lines().times(1).returning(|_| {
		Ok(vec![
			"a,Name Surname,someone@users.noreply.github.com".to_string(),
			"b,username,something@gmail.com".to_string(),
			"b,username2,something2@gmail.com".to_string(),
		])
	});
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["b".to_string()]);

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn provide_no_author_when_alias_doesnt_match() {
	let mut mock_file_loader = MockReader::new();
	mock_file_loader
		.expect_read_non_empty_lines()
		.with(predicate::always())
		.times(1)
		.returning(|_| Ok(vec!["a,Name Surname,someone@users.noreply.github.com".to_string()]));
	let provider = load_from_cwd(&mock_file_loader).expect("Could not setup FSProvider for test");

	let retrieved_authors = provider.find(vec!["z".to_string()]);

	assert_eq!(retrieved_authors.len(), 0);
}

fn load_from_path(file_reader: &MockReader) -> Result<CSVReader> {
	CSVReader::load(&LoadMode::FromPath {
		file_reader,
		path: PathBuf::from(IRRELEVANT_FILE_PATH),
	})
}

fn load_from_cwd(file_reader: &MockReader) -> Result<CSVReader> {
	CSVReader::load(&LoadMode::FromCwd { file_reader })
}
