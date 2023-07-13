use crate::author::domain::{Author, Repository};

use super::FSRepo;

#[test]
fn should_read_file() {
    let repo = FSRepo::new("tests/data/file_to_read");
    let contents = repo.read_file();

    assert!(contents.contains(&String::from("hi")));
    assert!(contents.contains(&String::from("mom")));
}

#[test]
fn should_fetch_all_available_authors() {
    let repo = FSRepo::new("tests/data/authors");

    let expected_authors = Vec::from([
        Author::new("a", "Name Surname", "someone@users.noreply.github.com"),
        Author::new("b", "username", "something@gmail.com"),
    ]);

    let actual_authors = repo.all_authors();

    assert_eq!(actual_authors, expected_authors);
}

#[test]
fn should_fetch_authors_based_on_alias() {
    let repo = FSRepo::new("tests/data/authors");

    let alias = "b";
    let expected_authors = Vec::from([Author::new(alias, "username", "something@gmail.com")]);

    let actual_authors = repo.find_authors(Vec::from([alias]));

    assert_eq!(actual_authors, expected_authors);
}
