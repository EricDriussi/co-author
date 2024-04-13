use crate::common::fs::file_writer::{FileWriter, Writer};
use crate::common::fs::test::util::random_tmp_file;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

const EXPECTED: &str = "some text sample";

#[test]
fn overwrite_file_content() {
	let (_, path) = random_tmp_file::create();

	let writer = FileWriter;
	let result = writer.overwrite(&PathBuf::from(path.clone()), EXPECTED);

	let mut actual = String::new();
	BufReader::new(
		OpenOptions::new()
			.read(true)
			.open(path.clone())
			.expect("Could not open test file"),
	)
	.read_to_string(&mut actual)
	.expect("Something went wrong in file writer test");
	fs::remove_file(path).expect("Could not cleanup file");

	assert!(result.is_ok());
	assert_eq!(actual, EXPECTED);
}

#[test]
fn create_file_when_overwriting() {
	let path_to_no_file = random_tmp_file::path();

	let writer = FileWriter;
	let result = writer.overwrite(&PathBuf::from(path_to_no_file.as_str()), EXPECTED);

	let mut actual = String::new();
	BufReader::new(
		OpenOptions::new()
			.read(true)
			.open(path_to_no_file.as_str())
			.expect("Could not open test file"),
	)
	.read_to_string(&mut actual)
	.expect("Something went wrong in file writer test");
	fs::remove_file(path_to_no_file).expect("Could not cleanup file");

	assert!(result.is_ok());
	assert_eq!(actual, EXPECTED);
}

#[test]
fn append_to_file() {
	let (mut file, path) = random_tmp_file::create();
	let pre_existing_content = EXPECTED;
	file.write_all(pre_existing_content.as_bytes())
		.expect("Could not write to file for test");

	let writer = FileWriter;
	let expected_new_content = "some more text!";
	let result = writer.append(&PathBuf::from(path.clone()), expected_new_content);

	let mut actual = String::new();
	BufReader::new(
		OpenOptions::new()
			.read(true)
			.open(path.clone())
			.expect("Could not open test file"),
	)
	.read_to_string(&mut actual)
	.expect("Something went wrong in file writer test");
	fs::remove_file(path).expect("Could not cleanup file");

	assert!(result.is_ok());
	assert_eq!(actual, format!("{pre_existing_content}{expected_new_content}"));
}

#[test]
fn error_when_appending_to_a_non_existent_file() {
	let path_to_no_file = random_tmp_file::path();

	let writer = FileWriter;
	let result = writer.append(&PathBuf::from(path_to_no_file), EXPECTED);

	assert!(result.is_err());
}
