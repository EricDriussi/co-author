use std::fs;

use crate::common::fs::file::SimpleFile;

#[test]
fn should_build_from_path_when_file_is_present() {
	let file = SimpleFile::from("src/common/fs/test/test_file.txt".to_string());

	assert!(file.is_some());
}

#[test]
fn should_not_build_from_path_when_file_is_absent() {
	let file = SimpleFile::from("not/a/real/path/whatever.something".to_string());

	assert!(file.is_none());
}

#[test]
fn should_create_when_file_is_absent() {
	let file_path = "/tmp/coa_test_file_create.txt";
	let file = SimpleFile::open_or_create(file_path.to_string());

	fs::remove_file(file_path).expect("Could not cleanup files");
	assert!(file.is_some());
}

#[test]
fn should_present_non_empty_lines() {
	let file = SimpleFile::from("src/common/fs/test/test_file.txt".to_string()).expect("Could not open test file");

	let non_empyt_lines = file.non_empty_lines();

	assert_eq!(non_empyt_lines.len(), 3);
}

#[test]
fn should_keep_track_of_path() {
	let path = "src/common/fs/test/test_file.txt".to_string();
	let file = SimpleFile::from(path.clone()).expect("Could not open test file");

	let found_path = file.path();

	assert_eq!(found_path, path);
}

#[test]
fn should_write_to_file() {
	let path = "/tmp/coa_test_file_write.txt".to_string();
	let mut file = SimpleFile::open_or_create(path.clone()).expect("Could not create test file");

	let result = file.write("test123".to_string());

	fs::remove_file(path).expect("Could not cleanup files");
	assert!(result.is_ok());
}
