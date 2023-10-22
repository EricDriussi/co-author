use std::fs;

use serial_test::serial;

use crate::{
	authors::{self, author::Author},
	conf,
	test_utils::file_cleanup::AfterAssert,
};

#[test]
#[serial]
fn authors_module_should_setup_repo_from_default_file_path_if_present() {
	assert!(authors::new_fs_default_setup().is_err());

	let default_authors_file_path = conf::authors_file_path();
	fs::File::create(&default_authors_file_path).unwrap();
	let _after = AfterAssert::cleanup(&[default_authors_file_path.as_str()]);

	assert!(authors::new_fs_default_setup().is_ok());
}

#[test]
fn authors_module_should_setup_repo_from_given_file_path_if_present() {
	assert!(authors::fs_setup_from_file("/tmp/not_real".to_string()).is_err());

	let result = authors::fs_setup_from_file(conf::dummy_data());

	assert!(result.is_ok_and(|service| service.all_available()
		== [
			Author::new("a", "Name Surname", "someone@users.noreply.github.com"),
			Author::new("b", "username", "something@gmail.com"),
			Author::new("b", "username2", "something2@gmail.com"),
			Author::new("ab", "Another Surname", "someone@something.hi"),
		]));
}
