use crate::conf;
use std::fs;

use serial_test::serial;

use crate::authors::author::Author;
use crate::authors::author::AuthorsRepo;
use crate::authors::fs_repo::FSRepo;

#[test]
fn should_init_from_a_given_existing_file() {
	assert!(FSRepo::from(conf::authors_file()).is_ok());
}

#[test]
fn should_not_init_if_a_given_file_does_not_exist() {
	assert!(FSRepo::from("no_file_here".to_string()).is_err());
}

#[test]
#[serial]
fn should_look_in_cwd_when_no_authors_file_is_found_in_default_path() {
	let authors_file_in_cwd = "./authors";
	fs::File::create(authors_file_in_cwd).unwrap();

	let repo = FSRepo::default("/not_real".to_string());
	assert!(repo.is_ok());

	fs::remove_file(authors_file_in_cwd).unwrap();
}

#[test]
#[serial]
fn should_fail_to_init_when_no_valid_file_is_found() {
	assert!(FSRepo::default("/not_real".to_string()).is_err());
}

#[test]
fn should_fetch_all_available_authors() {
	let repo = FSRepo::from(conf::authors_file()).unwrap();

	let actual_authors = repo.all();

	let expected_authors = Vec::from([
		Author::new("a", "Name Surname", "someone@users.noreply.github.com"),
		Author::new("b", "username", "something@gmail.com"),
		Author::new("b", "username2", "something2@gmail.com"),
		Author::new("ab", "Another Surname", "someone@something.hi"),
	]);
	assert_eq!(actual_authors, expected_authors);
}

#[test]
fn should_fetch_authors_based_on_alias() {
	let repo = FSRepo::from(conf::authors_file()).unwrap();

	let alias = "a";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	let expected_authors = Vec::from([Author::new(alias, "Name Surname", "someone@users.noreply.github.com")]);
	assert_eq!(actual_authors, expected_authors);
}

#[test]
fn should_fetch_all_authors_for_a_given_alias() {
	let repo = FSRepo::from(conf::authors_file()).unwrap();

	let alias = "b";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	let expected_authors = Vec::from([
		Author::new(alias, "username", "something@gmail.com"),
		Author::new(alias, "username2", "something2@gmail.com"),
	]);
	assert_eq!(actual_authors, expected_authors);
}

#[test]
fn should_return_an_empty_list_if_no_author_mathces_alias() {
	let repo = FSRepo::from(conf::authors_file()).unwrap();

	let alias = "z";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	let expected_authors = Vec::from([]);
	assert_eq!(actual_authors, expected_authors);
}
