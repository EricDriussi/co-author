use std::{
	fs::{self, OpenOptions},
	io::Write,
};

use crate::common::fs::{file::SimpleFile, test::create_random_tmp_file};

#[test]
fn should_present_non_empty_lines() {
	let (mut file, path) = create_random_tmp_file();
	file.write_all(b"one\n\ntwo\n\nthree\n\n")
		.expect("Could not write to file for test");
	let file = SimpleFile::from(
		OpenOptions::new()
			.read(true)
			.append(true)
			.open(path.clone())
			.expect("Could not open test file"),
		path.clone(),
	);

	let non_empyt_lines = file.non_empty_lines();

	fs::remove_file(path).expect("Could not cleanup file for test");
	assert_eq!(non_empyt_lines.len(), 3);
}

#[test]
fn should_keep_track_of_path() {
	let (_, path) = create_random_tmp_file();
	let file = SimpleFile::from(
		OpenOptions::new()
			.read(true)
			.append(true)
			.open(path.clone())
			.expect("Could not open test file"),
		path.clone(),
	);

	let found_path = file.path();

	assert_eq!(found_path, path);
}

#[test]
fn should_write_to_file() {
	let (_, path) = create_random_tmp_file();
	let mut file = SimpleFile::from(
		OpenOptions::new()
			.read(true)
			.append(true)
			.create(true)
			.open(path.clone())
			.expect("Could not create test file"),
		path.clone(),
	);

	let result = file.write("test123".to_string());

	fs::remove_file(path).expect("Could not cleanup file");
	assert!(result.is_ok());
}
