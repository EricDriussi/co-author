use std::io::{sink, Cursor};

use authors::{
    app_service::AuthorsService,
    author::{Author, AuthorsRepo},
};
use co_author::{cli::Cli, run_with_cli};
use git::{
    app_service::GitService,
    git::{CommitBody, GitRepo},
};

// TODO.Also test without mocks? Only mock cli?
#[test]
fn mocked_cli_flow() {
    let git_service = GitService::new(MockGitRepo::new());
    let authors_service = AuthorsService::new(MockAuthorRepo::new());
    let nothing = sink();
    let raw_input = Cursor::new(format!(
        "{}{}{}{}",
        "a commit message", "\n", "a b cd", "\n"
    ));
    let cli = Cli::new(raw_input, nothing);

    let result = run_with_cli(git_service, authors_service, cli);

    assert!(result.is_ok());
}

struct MockGitRepo {}

impl MockGitRepo {
    fn new() -> Self {
        Self {}
    }
}

impl GitRepo for MockGitRepo {
    fn commit(&self, _body: CommitBody) -> Result<(), String> {
        return Ok(());
    }
}

struct MockAuthorRepo {}

impl MockAuthorRepo {
    fn new() -> Self {
        Self {}
    }
}

impl AuthorsRepo for MockAuthorRepo {
    fn find(&self, _aliases: Vec<String>) -> Vec<Author> {
        return Vec::from([
            Author::new("a", "John", "Doe"),
            Author::new("b", "Jane", "Smith"),
        ]);
    }

    fn all(&self) -> Vec<Author> {
        return Vec::from([
            Author::new("a", "John", "Doe"),
            Author::new("b", "Jane", "Smith"),
        ]);
    }
}
