use crate::conf;
use crate::test_utils::file_cleanup::AfterAssert;
use crate::test_utils::set_test_env;

use std::fs;

use serial_test::serial;

use crate::authors::author::Author;
use crate::authors::author::AuthorsProvider;
use crate::authors::fs_provider::FSProvider;

#[test]
#[serial]
fn should_connect_to_an_authors_file_in_cwd_if_available() {
	set_test_env();
	let cwd_authors_file_path = conf::authors_file_name();
	fs::File::create(cwd_authors_file_path.clone()).unwrap();
	let _after = AfterAssert::cleanup_file(&&cwd_authors_file_path);

	assert!(FSProvider::from_cwd_with_home_fallback().is_ok());
}

#[test]
#[serial]
fn should_connect_to_the_default_authors_file_if_no_file_is_available_in_cwd() {
	set_test_env();
	let default_authors_file_path = conf::authors_file_path();
	fs::File::create(&default_authors_file_path).unwrap();
	let _after = AfterAssert::cleanup_file(&default_authors_file_path.as_str());

	assert!(FSProvider::from_cwd_with_home_fallback().is_ok());
}

#[test]
#[serial]
fn should_error_when_neither_cwd_or_default_authors_file_are_available() {
	set_test_env();
	assert!(FSProvider::from_cwd_with_home_fallback().is_err());
}

#[test]
fn should_connect_to_a_given_existing_authors_file() {
	set_test_env();
	let an_authors_file_path = "/tmp/an_authors_file";
	fs::File::create(an_authors_file_path).unwrap();
	let _after = AfterAssert::cleanup_file(&an_authors_file_path);

	assert!(FSProvider::from(an_authors_file_path.to_string()).is_ok());
}

#[test]
fn should_not_connect_to_a_given_non_existing_file() {
	set_test_env();
	assert!(FSProvider::from("/tmp/no_file_here".to_string()).is_err());
}

#[test]
fn should_fetch_all_available_authors() {
	set_test_env();
	let an_authors_file_path = conf::dummy_data();
	let repo = FSProvider::from(an_authors_file_path.to_string()).unwrap();

	let actual_authors = repo.all();

	assert_eq!(
		actual_authors,
		[
			Author::from("a", "Name Surname", "someone@users.noreply.github.com"),
			Author::from("b", "username", "something@gmail.com"),
			Author::from("b", "username2", "something2@gmail.com"),
			Author::from("ab", "Another Surname", "someone@something.hi"),
		]
	);
}

#[test]
fn should_fetch_authors_based_on_alias() {
	set_test_env();
	let an_authors_file_path = conf::dummy_data();
	let repo = FSProvider::from(an_authors_file_path.to_string()).unwrap();

	let alias = "a";
	let actual_author = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_author,
		[Author::from(alias, "Name Surname", "someone@users.noreply.github.com")]
	);
}

#[test]
fn should_fetch_all_authors_for_a_given_alias() {
	set_test_env();
	let an_authors_file_path = conf::dummy_data();
	let repo = FSProvider::from(an_authors_file_path.to_string()).unwrap();

	let alias = "b";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_authors,
		[
			Author::from(alias, "username", "something@gmail.com"),
			Author::from(alias, "username2", "something2@gmail.com"),
		]
	);
}

#[test]
fn should_return_an_empty_list_if_no_author_mathces_alias() {
	set_test_env();
	let an_authors_file_path = conf::dummy_data();
	let repo = FSProvider::from(an_authors_file_path.to_string()).unwrap();

	let alias = "z";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(actual_authors, []);
}
