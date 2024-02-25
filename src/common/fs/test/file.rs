use std::fs::{self, OpenOptions};

use crate::common::fs::file::SimpleFile;

#[test]
fn should_present_non_empty_lines() {
	let path = "src/common/fs/test/test_file.txt".to_string();
	let file = SimpleFile::from(
		OpenOptions::new()
			.read(true)
			.append(true)
			.open(path.clone())
			.expect("Could not open test file"),
		path.clone(),
	);

	let non_empyt_lines = file.non_empty_lines();

	assert_eq!(non_empyt_lines.len(), 3);
}

#[test]
fn should_keep_track_of_path() {
	let path = "src/common/fs/test/test_file.txt".to_string();
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
	let path = "/tmp/coa_test_file_write.txt".to_string();
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
