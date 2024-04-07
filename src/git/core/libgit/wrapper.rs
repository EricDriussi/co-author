use super::status_formatter;
use crate::common::conf;
use crate::common::fs::file_reader::Reader;
use crate::git::core::commit_message::{CommitMessage, GitWrapper};
use crate::git::err::GitError;
use crate::Result;
use git2::{Repository, Signature};
use std::path::PathBuf;

pub struct LibGitWrapper<R: Reader> {
	repo: Repository,
	path: PathBuf,
	reader: R,
}

impl<R: Reader> GitWrapper for LibGitWrapper<R> {
	fn commit(&self) -> Result<()> {
		let signature = self
			.repo
			.signature()
			.map_err(|_| GitError::LibGit("User name and/or email not set".to_string()))?;

		let commit_message = CommitMessage::from(
			&self
				.reader
				.read_non_empty_lines(&self.path.join(conf::editmsg()))
				.unwrap_or_default()
				.join("\n"),
		);

		if commit_message.has_no_content() {
			return Err(Box::new(GitError::LibGit("Commit message cannot be empty".to_string())));
		}

		self.add_commit(&signature, &commit_message.to_string())?;
		Ok(())
	}

	fn formatted_status(&self) -> Result<String> {
		status_formatter::get_status_for_commit_file(&self.repo)
	}

	fn prev_commit_msg(&self) -> Result<CommitMessage> {
		let last_commit = self.repo.head().and_then(|head_ref| head_ref.peel_to_commit())?;

		Ok(CommitMessage::from(last_commit.message().unwrap_or_default()))
	}
}

impl<R: Reader> LibGitWrapper<R> {
	pub fn from(path: &PathBuf, file_reader: R) -> Result<Self> {
		let repo = Repository::discover(path).map_err(|_| GitError::LibGit("Could not open git repo".to_string()))?;
		if Self::no_staged_changes(&repo) {
			Err(Box::new(GitError::LibGit("No staged changes".to_string())))
		} else {
			Ok(Self {
				repo,
				path: path.clone(),
				reader: file_reader,
			})
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

	fn add_commit(&self, signature: &Signature, commit_message: &str) -> Result<()> {
		let tree = self.repo.find_tree(self.repo.index()?.write_tree()?)?;

		match self.repo.head() {
			Ok(head_ref) => self.repo.commit(
				Some("HEAD"),
				signature,
				signature,
				commit_message,
				&tree,
				&[&head_ref.peel_to_commit()?],
			),
			Err(_) => self
				.repo
				.commit(Some("HEAD"), signature, signature, commit_message, &tree, &[]),
		}
		.map(|_| ())
		.map_err(Into::into)
	}
}
