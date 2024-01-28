use super::*;

// TODO: if env test (set_test_env()) is no longer needed, change the assertions to match the prod config values
#[test]
#[ignore]
fn should_return_expected_test_authors_file_path() {
	let authors_file_path = config::authors_dir();
	assert_eq!(authors_file_path, "src/authors/test/data/authors");
}

#[test]
#[ignore]
fn should_return_expected_test_authors_file_name() {
	let authors_file_path = config::authors_file();
	assert_eq!(authors_file_path, "authors");
}

#[test]
#[ignore]
fn should_return_expected_test_hooks_path() {
	let hooks_path = config::hooks_path();
	assert_eq!(hooks_path, ".git/hooks/");
}

#[test]
#[ignore]
fn should_return_expected_test_editmsg_file() {
	let commit_editmsg = config::editmsg();
	assert_eq!(commit_editmsg, "/var/tmp/coa/.git/COMMIT_EDITMSG");
}
