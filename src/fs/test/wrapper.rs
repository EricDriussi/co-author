use crate::fs::wrapper::{FileLoader, FsWrapper};

#[test]
fn should_load_some_file_from_cwd() {
	let wrapper = FsWrapper::new();

	let actual_file = wrapper.load_file("src/fs/test/test_file.txt".to_string());

	assert!(actual_file.is_some());
}
