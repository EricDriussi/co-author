use std::fs;

use crate::common::fs::{
	test::create_random_tmp_file,
	wrapper::{FileLoader, FsWrapper},
};

#[test]
fn should_load_file_if_present() {
	let (_, file_path) = create_random_tmp_file();
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_if_present(file_path.to_string());

	fs::remove_file(file_path).expect("Could not cleanup file for test");
	assert!(actual_file.is_some());
}

#[test]
fn should_not_load_file_if_absent() {
	let (_, file_path) = create_random_tmp_file();
	fs::remove_file(file_path.clone()).expect("Could not cleanup file for test");
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_if_present(file_path.to_string());

	assert!(actual_file.is_none());
}

#[test]
fn should_create_file_when_absent() {
	let wrapper = FsWrapper::new();
	let (_, file_path) = create_random_tmp_file();
	fs::remove_file(file_path.clone()).expect("Could not cleanup file for test");

	let actual_file = wrapper.load(file_path.to_string());

	fs::remove_file(file_path).expect("Could not cleanup file for test");
	assert!(actual_file.is_some());
}
