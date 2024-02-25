use crate::common::fs::wrapper::{FileLoader, FsWrapper};

#[test]
fn should_load_some_file_from_cwd() {
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_if_present("src/common/fs/test/test_file.txt".to_string());

	assert!(actual_file.is_some());
}
