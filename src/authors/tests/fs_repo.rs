use authors::{
    author::{Author, AuthorRepo},
    fs_repo::FSRepo,
};

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

    let actual_authors = repo.find_authors(Vec::from([String::from(alias)]));

    assert_eq!(actual_authors, expected_authors);
}
