use co_author::authors::{application::AuthService, infrastructure::FSRepo};

#[test]
fn authors() {
    let repo = FSRepo::new("tests/data/authors");
    let app_service = AuthService::new(repo);

    let authors = app_service.find_authors(Vec::from([String::from("a")]));

    assert_eq!(authors.len(), 1);
}
