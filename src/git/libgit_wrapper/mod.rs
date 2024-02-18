use super::{
	commit_message::{CommitMessage, GitWrapper},
	git_err::GitError,
};
use crate::common::conf;
use crate::Result;
use git2::{Repository, Signature};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub mod editmsg_handler;

pub struct LibGitWrapper {
	repo: Repository,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self) -> Result<()> {
		let signature = self
			.repo
			.signature()
			.map_err(|_| GitError::LibGit("User name and/or email not set".to_string()))?;

		let commit_message =
			editmsg_handler::read_editmsg().ok_or(GitError::LibGit("Commit message cannot be empty".to_string()))?;

		self.try_to_commit(&signature, &commit_message)
			.map_err(|_| GitError::LibGit("Something went wrong!".to_string()))?;
		Ok(())
	}

	fn write_to_editmsg(&self, commit_message: &CommitMessage) -> Result<()> {
		editmsg_handler::write_commit_to_file(commit_message)
	}

	fn add_status_to_editmsg(&self) -> Result<()> {
		let status = editmsg_handler::get_status_for_commit_file(&self.repo);

		let mut file_to_append = OpenOptions::new().create(true).append(true).open(conf::editmsg())?;
		file_to_append.write_all(status.as_bytes())?;
		Ok(())
	}

	fn prev_commit_msg(&self) -> Result<String> {
		let head_ref = self.repo.head()?;
		let last_commit = head_ref.peel_to_commit()?;

		let commit_message = last_commit.message().unwrap_or_default();
		let first_line = commit_message.lines().next().unwrap_or_default();

		Ok(first_line.to_string())
	}
}

impl LibGitWrapper {
	pub fn from(path: &PathBuf) -> Result<Self> {
		let repo = Repository::open(path).map_err(|_| GitError::LibGit("Could not open git repo".to_string()))?;
		if Self::no_staged_changes(&repo) {
			Err(Box::new(GitError::LibGit("No staged changes".to_string())))
		} else {
			Ok(Self { repo })
		}
	}

	fn no_staged_changes(repo: &Repository) -> bool {
		if let Ok(head) = repo.head() {
			if let Ok(tree) = head.peel_to_tree() {
				if let Ok(index) = repo.index() {
					if let Ok(diff) = repo.diff_tree_to_index(Some(&tree), Some(&index), None) {
						return diff.deltas().count() == 0;
					}
				}
			}
		}
		false
	}

	fn try_to_commit(&self, signature: &Signature, commit_message: &str) -> Result<()> {
		let oid = self.repo.index()?.write_tree()?;
		let tree = self.repo.find_tree(oid)?;
		let parent_commit = self.repo.head()?.peel_to_commit()?;
		Ok(self
			.repo
			.commit(
				Some("HEAD"),
				signature,
				signature,
				commit_message,
				&tree,
				&[&parent_commit],
			)
			.map(|_| ())?)
	}
}
