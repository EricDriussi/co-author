use super::domain::{CommitBody, GitRepo};

struct Service<T: GitRepo> {
    repo: T,
}

impl<T: GitRepo> Service<T> {
    fn new(repo: T) -> Service<T> {
        Service { repo }
    }

    pub fn is_valid_git_repo(&self, path: String) -> bool {
        return T::is_valid(path);
    }

    pub fn commit(&self, message: &str, aliases: Vec<String>) -> Result<(), String> {
        return self.repo.commit(CommitBody::new(message, aliases));
    }
}

pub fn run() -> bool {
    return true;
}

#[cfg(test)]
mod test;
