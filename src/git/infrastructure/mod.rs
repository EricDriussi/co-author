use git2::{Repository, Signature};

use super::domain::{CommitBody, GitRepo};

pub struct Git2Repo {
    repo: Repository,
}

impl GitRepo for Git2Repo {
    fn commit(&self, commit_body: CommitBody) -> Result<(), String> {
        match self.no_staged_changes() {
            Ok(no_changes) => {
                if no_changes {
                    return Err(String::from("No changes staged for commit"));
                }
            }
            Err(_) => return Err(String::from("Something went wrong!")),
        };

        let signature = match self.repo.signature() {
            Ok(sig) => sig,
            Err(_) => return Err(String::from("User name and/or email not set")),
        };

        match self.try_to_commit(signature, commit_body) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(String::from("Something went wrong!")),
        };
    }
}

impl Git2Repo {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    fn no_staged_changes(&self) -> Result<bool, git2::Error> {
        let head = self.repo.head()?;
        let tree = head.peel_to_tree()?;
        let index = self.repo.index()?;
        let diff = self
            .repo
            .diff_tree_to_index(Some(&tree), Some(&index), None)?;
        return Ok(diff.deltas().count() == 0);
    }

    fn try_to_commit(
        &self,
        signature: Signature,
        commit_body: CommitBody,
    ) -> Result<(), git2::Error> {
        let authors_string = commit_body.get_signatures().join("\n");
        let formatted_commit_message =
            format!("{}\n\n{}", commit_body.get_message(), authors_string);

        let oid = self.repo.index()?.write_tree()?;
        let tree = self.repo.find_tree(oid)?;
        let parent_commit = self.repo.head()?.peel_to_commit()?;
        self.repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                &formatted_commit_message,
                &tree,
                &[&parent_commit],
            )
            .map(|_| ())
    }
}

#[cfg(test)]
mod test;
