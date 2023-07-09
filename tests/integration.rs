use co_author::{authors, git};

#[test]
fn authors_runs() {
    assert!(authors::run());
}

#[test]
fn git_runs() {
    assert!(git::run());
}
