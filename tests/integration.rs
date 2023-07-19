use co_author::authors::{application::Service, infrastructure::FSRepo};

#[test]
fn authors() {
    let repo = FSRepo::new("tests/data/authors");
    let app_service = Service::new(repo);

    let authors = app_service.find_authors(Vec::from([String::from("a")]));

    assert_eq!(authors.len(), 1);
}

#[test]
fn git_runs() {
    assert!(git::app_service::run());
}
