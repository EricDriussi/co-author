use crate::common::fs::{
	file_reader::{Reader, SimpleReader},
	test::util::random_tmp_file,
};
use std::{fs, io::Write, path::PathBuf};

#[test]
fn read_non_empty_lines() {
	let (mut file, path) = random_tmp_file::create();
	file.write_all(b"one\n\ntwo\n\nthree\n\n")
		.expect("Could not write to file for test");
	let reader = SimpleReader::new();

	let non_empyt_lines = reader.read_non_empty_lines(&PathBuf::from(path.clone()));

	fs::remove_file(path).expect("Could not cleanup file for test");
	assert!(non_empyt_lines.is_ok());
	assert_eq!(
		non_empyt_lines.expect("Something went wrong in file reader test").len(),
		3
	);
}

#[test]
fn error_when_reading_lines_from_non_existent_file() {
	let reader = SimpleReader::new();
	let path_to_no_file = random_tmp_file::path();

	let non_empyt_lines = reader.read_non_empty_lines(&PathBuf::from(path_to_no_file.as_str()));

	assert!(non_empyt_lines.is_err());
}
