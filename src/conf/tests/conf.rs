use std::path::PathBuf;

#[test]
fn should_return_expected_test_authors_file_path() {
	let authors_file_path = PathBuf::from(conf::authors_file());
	assert_eq!(authors_file_path.to_str().unwrap(), "src/authors/tests/data/authors");
}

#[test]
fn should_return_expected_test_hooks_path() {
	let authors_file_path = PathBuf::from(conf::hooks_path());
	assert_eq!(authors_file_path.to_str().unwrap(), "../../.git/hooks");
}
