use authors::{
    author::{Author, AuthorsRepo},
    fs_repo::FSRepo,
};

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
