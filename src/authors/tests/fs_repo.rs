use std::fs;
use authors::{
    author::{Author, AuthorsRepo},
    fs_repo::FSRepo,
};

#[test]
fn should_init_from_a_given_existing_file() {
	let repo = FSRepo::from(Some("tests/data/authors".to_string()));
	assert!(repo.is_ok());
}

#[test]
fn should_not_init_if_a_given_file_is_non_existent() {
	let repo = FSRepo::from(Some("no_file_here".to_string()));
	assert!(repo.is_err());
}

#[test]
fn should_default_to_cwd_when_looking_for_a_authors_file(){
	let authors_file_in_cwd = "./authors";
	fs::File::create(authors_file_in_cwd).unwrap();

	let repo = FSRepo::from(None);
	assert!(repo.is_ok());

	fs::remove_file(authors_file_in_cwd).unwrap();
}

#[test]
fn should_fail_to_init_when_no_valid_file_is_found(){
	let repo = FSRepo::from(None);
	assert!(repo.is_err());
}


#[test]
fn should_fetch_all_available_authors() {
    let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();

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
    let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();

    let alias = "a";
    let actual_authors = repo.find(Vec::from([String::from(alias)]));

    let expected_authors = Vec::from([Author::new(
        alias,
        "Name Surname",
        "someone@users.noreply.github.com",
    )]);
    assert_eq!(actual_authors, expected_authors);
}

#[test]
fn should_fetch_all_authors_for_a_given_alias() {
    let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();

    let alias = "b";
    let actual_authors = repo.find(Vec::from([String::from(alias)]));

    let expected_authors = Vec::from([
        Author::new(alias, "username", "something@gmail.com"),
        Author::new(alias, "username2", "something2@gmail.com"),
    ]);
    assert_eq!(actual_authors, expected_authors);
}
