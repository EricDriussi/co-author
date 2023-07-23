use crate::git::{CommitBody, GitRepo};

pub struct GitService<T: GitRepo> {
    repo: T,
}

impl<T: GitRepo> GitService<T> {
    pub fn new(repo: T) -> GitService<T> {
        GitService { repo }
    }

    pub fn commit(&self, message: &str, aliases: Vec<String>) -> Result<(), String> {
        return self.repo.commit(CommitBody::new(message, aliases));
    }
}
