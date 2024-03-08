use crate::common::conf;

#[test]
fn should_return_expected_test_authors_file_path() {
	let authors_dir = conf::authors_dir();
	assert_eq!(authors_dir, "co-author");
}

#[test]
fn should_return_expected_test_authors_file_name() {
	let authors_file = conf::authors_file();
	assert_eq!(authors_file, "authors.csv");
}

#[test]
fn should_return_expected_test_hooks_path() {
	let hooks_path = conf::hooks_path();
	assert_eq!(hooks_path, ".git/hooks");
}

#[test]
fn should_return_expected_test_editmsg_file() {
	let commit_editmsg = conf::editmsg();
	assert_eq!(commit_editmsg, ".git/COMMIT_EDITMSG");
}
