use authors::{app_service::AuthorsService, fs_repo::FSRepo};

#[test]
fn authors() {
    let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();
    let app_service = AuthorsService::new(repo);

    let authors = app_service.signatures_of(Vec::from([String::from("a")]));

    assert_eq!(authors.len(), 1);
}
