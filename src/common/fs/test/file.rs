use crate::common::fs::file::File;

#[test]
fn should_build_from_path_when_file_is_present() {
	let file = File::from("src/common/fs/test/test_file.txt".to_string());

	assert!(file.is_some());
}

#[test]
fn should_not_build_from_path_when_file_is_absent() {
	let file = File::from("not/a/real/path/whatever.something".to_string());

	assert!(file.is_none());
}

#[test]
fn should_present_non_empty_lines() {
	let file = File::from("src/common/fs/test/test_file.txt".to_string()).expect("Could not open test file");

	let non_empyt_lines = file.non_empty_lines();

	assert_eq!(non_empyt_lines.len(), 3);
}

#[test]
fn should_keep_track_of_path() {
	let path = "src/common/fs/test/test_file.txt".to_string();
	let file = File::from(path.clone()).expect("Could not open test file");

	let found_path = file.path();

	assert_eq!(found_path, path);
}
