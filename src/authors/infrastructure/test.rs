use crate::authors::domain::{Author, Repository};

use super::FSRepo;

#[test]
fn should_read_lines() {
    let repo = FSRepo::new("tests/data/authors");
    let contents = repo.read_lines();

    assert!(contents.is_ok());
}

#[test]
fn should_filter_by_alias() {
    let fs_repo = FSRepo::new("no_file_needed");

    let matching_alias = fs_repo.filter_by_alias("a,John,Doe", &["a"]);
    assert_eq!(matching_alias, true);

    let no_matching_alias = fs_repo.filter_by_alias("b,Jane,Dane", &["a"]);
    assert_eq!(no_matching_alias, false);
}

#[test]
fn should_parse_author() {
    let fs_repo = FSRepo::new("no_file_needed");

    let valid_result = fs_repo.parse_author("a,John,Doe");
    assert_eq!(valid_result, Some(Author::new("a", "John", "Doe")));

    let invalid_result = fs_repo.parse_author("hi,invalid_line");
    assert_eq!(invalid_result, None);
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
