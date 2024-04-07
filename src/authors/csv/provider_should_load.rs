use super::provider::LoadMode;
use crate::authors::csv::provider::CSVProvider;
use crate::authors::err::AuthorsError;
use crate::common::conf;
use crate::common::fs::file_reader::MockReader;
use crate::error::{assert_error_contains_msg, assert_error_type};
use crate::Result;
use mockall::predicate::eq;
use mockall::Sequence;
use serial_test::serial;
use std::path::PathBuf;

#[test]
fn load_from_given_file() {
	let mut mock_reader = MockReader::new();
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(String::new())))
		.returning(|_| Ok(vec![]));

	let result = CSVProvider::load(&LoadMode::FromPath {
		file_reader: &mock_reader,
		path: PathBuf::from(String::new()),
	});

	assert!(result.is_ok());
}

#[test]
fn not_load_from_given_file() {
	let irrelevant_file = "a/path/file.hi";
	let irrelevant_file_path = PathBuf::from(irrelevant_file);
	let mut mock_reader = MockReader::new();
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(irrelevant_file_path.clone()))
		.returning(|_| Err("oops".into()));

	let result = CSVProvider::load(&LoadMode::FromPath {
		file_reader: &mock_reader,
		path: irrelevant_file_path,
	});

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, irrelevant_file);
}

#[test]
fn load_using_fallback() {
	let mut mock_reader = MockReader::new();
	mock_reader.expect_read_non_empty_lines().returning(|_| Ok(vec![]));

	assert!(load_from_cwd(&mock_reader).is_ok());
}

#[test]
fn not_load_using_fallback() {
	let mut mock_reader = MockReader::new();
	mock_reader
		.expect_read_non_empty_lines()
		.returning(|_| Err("oops".into()));

	let result = load_from_cwd(&mock_reader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, "$PWD or $HOME");
}

#[serial]
fn fallback_sensibly() {
	let cwd = "/tmp";
	let xdg_config = "a_path";
	let home = "a_home";
	std::env::set_current_dir(PathBuf::from(cwd)).expect("Could not set current dir for tests");
	std::env::set_var("XDG_CONFIG_HOME", xdg_config);
	std::env::set_var("HOME", home);
	let file_path = &conf::authors_file();
	let dir_path = &conf::authors_dir();
	let mut seq = Sequence::new();
	let mut mock_reader = MockReader::new();
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{cwd}/{file_path}"))))
		.returning(|_| Err("oops".into()))
		.times(1)
		.in_sequence(&mut seq);
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{xdg_config}/{dir_path}/{file_path}"))))
		.returning(|_| Err("oops".into()))
		.times(1)
		.in_sequence(&mut seq);
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/.config/{dir_path}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/.{dir_path}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);
	mock_reader
		.expect_read_non_empty_lines()
		.with(eq(PathBuf::from(format!("{home}/{file_path}"))))
		.times(1)
		.returning(|_| Err("oops".into()))
		.in_sequence(&mut seq);

	let result = load_from_cwd(&mock_reader);

	assert_error_type(&result, &AuthorsError::NotFound(String::new()));
	assert_error_contains_msg(&result, "$PWD or $HOME");
}

fn load_from_cwd(file_reader: &MockReader) -> Result<CSVProvider> {
	CSVProvider::load(&LoadMode::FromCwd { file_reader })
}
