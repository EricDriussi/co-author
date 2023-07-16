use git2::{Repository, Signature};

use super::domain::{CommitBody, GitRepo};

pub struct Git2Repo {
    repo: Repository,
}

impl Git2Repo {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }
}

impl GitRepo for Git2Repo {
    fn commit(&self, commit_body: CommitBody) -> Result<(), git2::Error> {
        let authors_string = commit_body.get_signatures().join("\n");

        let formatted_commit_message =
            format!("{}\n\n{}", commit_body.get_message(), authors_string);

        // Get the default user signature
        let signature = match self.repo.signature() {
            Ok(sig) => sig,
            Err(_) => Signature::now("anon", "not_a_real@email.something")?,
        };

        // Check if there are any changes staged for commit
        let head = self.repo.head()?;
        let tree = head.peel_to_tree()?;
        let mut index = self.repo.index()?;
        let diff = self
            .repo
            .diff_tree_to_index(Some(&tree), Some(&index), None)?;
        if diff.deltas().count() == 0 {
            return Err(git2::Error::from_str("No changes staged for commit"));
        }

        // Create a new tree from the index
        let oid = index.write_tree()?;
        let new_tree = self.repo.find_tree(oid)?;

        // Create a new commit
        let parent_commit = head.peel_to_commit()?;
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &formatted_commit_message,
            &new_tree,
            &[&parent_commit],
        )?;

        return Result::Ok(());
    }
}

#[cfg(test)]
mod test;
