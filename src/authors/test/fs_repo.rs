use crate::conf;
use std::fs;

use serial_test::serial;

use crate::authors::author::Author;
use crate::authors::author::AuthorsRepo;
use crate::authors::fs_repo::FSRepo;

#[test]
#[serial]
fn should_connect_to_an_authors_file_in_cwd_if_available() {
	// FIXME: why strings instead of PathBufs?
	let cwd_authors_file_path = conf::authors_file_name();
	fs::File::create(cwd_authors_file_path.clone()).unwrap();
	let _after = AfterAssert::cleanup(&[&cwd_authors_file_path]);

	assert!(FSRepo::new_default().is_ok());
}

#[test]
#[serial]
fn should_connect_to_the_default_authors_file_if_no_file_is_available_in_cwd() {
	let default_authors_file_path = conf::authors_file_path();
	fs::File::create(&default_authors_file_path).unwrap();
	let _after = AfterAssert::cleanup(&[default_authors_file_path.as_str()]);

	assert!(FSRepo::new_default().is_ok());
}

#[test]
#[serial]
fn should_error_when_neither_cwd_or_default_authors_file_are_available() {
	assert!(FSRepo::new_default().is_err());
}

#[test]
fn should_connect_to_a_given_existing_authors_file() {
	let an_authors_file_path = "/tmp/an_authors_file";
	fs::File::create(an_authors_file_path).unwrap();
	let _after = AfterAssert::cleanup(&[an_authors_file_path]);

	assert!(FSRepo::from(an_authors_file_path.to_string()).is_ok());
}

#[test]
fn should_not_connect_to_a_given_non_existing_file() {
	assert!(FSRepo::from("/tmp/no_file_here".to_string()).is_err());
}

#[test]
fn should_fetch_all_available_authors() {
	let an_authors_file_path = conf::dummy_data();
	let repo = FSRepo::from(an_authors_file_path.to_string()).unwrap();

	let actual_authors = repo.all();

	assert_eq!(
		actual_authors,
		Vec::from([
			Author::new("a", "Name Surname", "someone@users.noreply.github.com"),
			Author::new("b", "username", "something@gmail.com"),
			Author::new("b", "username2", "something2@gmail.com"),
			Author::new("ab", "Another Surname", "someone@something.hi"),
		])
	);
}

#[test]
fn should_fetch_authors_based_on_alias() {
	let an_authors_file_path = conf::dummy_data();
	let repo = FSRepo::from(an_authors_file_path.to_string()).unwrap();

	let alias = "a";
	let actual_author = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_author,
		Vec::from([Author::new(alias, "Name Surname", "someone@users.noreply.github.com")])
	);
}

#[test]
fn should_fetch_all_authors_for_a_given_alias() {
	let an_authors_file_path = conf::dummy_data();
	let repo = FSRepo::from(an_authors_file_path.to_string()).unwrap();

	let alias = "b";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(
		actual_authors,
		Vec::from([
			Author::new(alias, "username", "something@gmail.com"),
			Author::new(alias, "username2", "something2@gmail.com"),
		])
	);
}

#[test]
fn should_return_an_empty_list_if_no_author_mathces_alias() {
	let an_authors_file_path = conf::dummy_data();
	let repo = FSRepo::from(an_authors_file_path.to_string()).unwrap();

	let alias = "z";
	let actual_authors = repo.find(Vec::from([String::from(alias)]));

	assert_eq!(actual_authors, Vec::from([]));
}

struct AfterAssert {
	files: Vec<String>,
}
impl AfterAssert {
	pub fn cleanup(files: &[&str]) -> Self {
		Self {
			files: files.iter().map(|f| f.to_string()).collect(),
		}
	}
}
impl Drop for AfterAssert {
	fn drop(&mut self) {
		for file in &self.files {
			fs::remove_file(file).unwrap()
		}
	}
}
