use co_author::{
    authors::{application::Service, infrastructure::FSRepo},
    git,
};

#[test]
fn authors() {
    let repo = FSRepo::new("tests/data/authors");
    let app_service = Service::new(repo);

    let authors = app_service.get_available_authors();

    assert!(authors.len() == 2);
}

#[test]
fn git_runs() {
    assert!(git::run());
}
