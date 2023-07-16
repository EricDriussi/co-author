use co_author::{author, git};

#[test]
fn author() {
    let repo = author::infrastructure::FSRepo::new("tests/data/authors");
    let app_service = author::application::Service::new(repo);

    let authors = app_service.get_available_authors();

    assert!(authors.len() == 2);
}

#[test]
fn git_runs() {
    assert!(git::run());
}
