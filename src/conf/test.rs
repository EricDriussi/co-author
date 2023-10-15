use super::*;

#[test]
fn should_return_expected_test_authors_file_path() {
	let authors_file_path = conf::authors_file();
	assert_eq!(authors_file_path, "src/authors/test/data/authors");
}

#[test]
fn should_return_expected_test_hooks_path() {
	let hooks_path = conf::hooks_path();
	assert_eq!(hooks_path, ".git/hooks");
}

#[test]
fn should_return_expected_test_editmsg_file() {
	let commit_editmsg = conf::editmsg();
	assert_eq!(commit_editmsg, "/var/tmp/coa/.git/COMMIT_EDITMSG");
}
