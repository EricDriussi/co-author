use std::fs;

use crate::common::fs::wrapper::{FileLoader, FsWrapper};

#[test]
fn should_load_file_if_present() {
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_if_present("src/common/fs/test/test_file.txt".to_string());

	assert!(actual_file.is_some());
}

#[test]
fn should_not_load_file_if_absent() {
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_if_present("not/a/real/path/whatever.something".to_string());

	assert!(actual_file.is_none());
}

#[test]
fn should_create_file_when_absent() {
	let wrapper = FsWrapper::new();
	let file_path = "/tmp/coa_test_file_create.txt";

	let actual_file = wrapper.load(file_path.to_string());

	fs::remove_file(file_path).expect("Could not cleanup file");
	assert!(actual_file.is_some());
}
