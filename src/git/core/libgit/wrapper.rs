use super::status_builder;
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
		let signature = self.validate_signature()?;
		let commit_message = self.validate_commit_message()?;

		match self.repo.head() {
			// If there is a HEAD, take it as parent
			Ok(head) => self.repo.commit(
				head.name(),
				&signature,
				&signature,
				&commit_message.to_string(),
				&self.get_tree()?,
				&[&head.peel_to_commit()?],
			),

			// If there is no HEAD, commit without parent
			// First commit (maybe detached HEAD?)
			Err(_) => self.repo.commit(
				Some("HEAD"),
				&signature,
				&signature,
				&commit_message.to_string(),
				&self.get_tree()?,
				&[],
			),
		}
		.map(|_| ())
		.map_err(Into::into)
	}

	fn amend(&self) -> Result<()> {
		let signature = self.validate_signature()?;
		let commit_message = self.validate_commit_message()?;
		let head = self.repo.head()?;

		head.peel_to_commit()?
			.amend(
				head.name(),
				Some(&signature),
				Some(&signature),
				None,
				Some(&commit_message.to_string()),
				Some(&self.get_tree()?),
			)
			.map(|_| ())
			.map_err(Into::into)
	}

	fn formatted_status(&self) -> Result<String> {
		status_builder::for_editmsg(&self.repo)
	}

	fn prev_commit_msg(&self) -> Result<CommitMessage> {
		let last_commit = self.repo.head().and_then(|head_ref| head_ref.peel_to_commit())?;
		Ok(CommitMessage::from(last_commit.message().unwrap_or_default()))
	}
}

impl<R: Reader> LibGitWrapper<R> {
	pub fn from(path: &PathBuf, file_reader: R) -> Result<Self> {
		let repo = Repository::discover(path).map_err(|_| GitError::LibGit("Could not open git repo".to_string()))?;
		if Self::no_staged_changes(&repo)? {
			Err(Box::new(GitError::LibGit("No staged changes".to_string())))
		} else {
			Ok(Self {
				repo,
				path: path.clone(),
				reader: file_reader,
			})
		}
	}

	fn no_staged_changes(repo: &Repository) -> Result<bool> {
		let tree = repo.head()?.peel_to_tree()?;
		let index = repo.index()?;
		let diff = repo.diff_tree_to_index(Some(&tree), Some(&index), None)?;
		Ok(diff.deltas().count() == 0)
	}

	fn validate_signature(&self) -> Result<Signature> {
		Ok(self
			.repo
			.signature()
			.map_err(|_| GitError::LibGit("User name and/or email not set".to_string()))?)
	}

	fn validate_commit_message(&self) -> Result<CommitMessage> {
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

		Ok(commit_message)
	}

	fn get_tree(&self) -> Result<git2::Tree> {
		Ok(self.repo.find_tree(self.repo.index()?.write_tree()?)?)
	}
}
