use std::cell::Cell;

use super::*;

#[test]
fn should_commit() {
    let spy = SpyRepo::new();
    let service = Service::new(spy);
    let commit_message = "something";
    let aliases = vec![String::from("a")];

    let result = service.commit(commit_message, aliases);

    assert!(result.is_ok());
    assert!(service.repo.commit_called.get());
}

struct SpyRepo {
    commit_called: Cell<bool>,
}

impl SpyRepo {
    fn new() -> Self {
        Self {
            commit_called: Cell::new(false),
        }
    }
}

impl GitRepo for SpyRepo {
    fn commit(&self, _body: CommitBody) -> Result<(), String> {
        self.commit_called.set(true);
        return Ok(());
    }

    fn is_valid(_path: String) -> bool {
        return true;
    }
}
