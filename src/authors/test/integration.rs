use std::fs;

use serial_test::serial;

use crate::{
	authors::{self, author::Author},
	conf,
	test_utils::{file_cleanup::AfterAssert, set_test_env},
};

#[test]
#[serial]
#[ignore] // do these tests make any sense?
fn authors_module_should_setup_repo_from_default_file_path_if_present() {
	assert!(authors::default().is_err());

	let default_authors_file_path = conf::authors_file_path();
	fs::File::create(&default_authors_file_path).expect("Could not setup test authors file");
	let _after = AfterAssert::cleanup_file(default_authors_file_path.as_str());

	assert!(authors::default().is_ok());
}

#[test]
fn authors_module_should_setup_repo_from_given_file_path_if_present() {
	set_test_env();
	assert!(authors::from_file("/tmp/not_real").is_err());

	let result = authors::from_file(&conf::dummy_data());

	assert!(result.is_ok_and(|service| service.all_authors()
		== [
			Author::from("a", "Name Surname", "someone@users.noreply.github.com"),
			Author::from("b", "username", "something@gmail.com"),
			Author::from("b", "username2", "something2@gmail.com"),
			Author::from("ab", "Another Surname", "someone@something.hi"),
		]));
}
