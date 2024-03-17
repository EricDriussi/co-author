use std::path::PathBuf;

use crate::git::commit_message::CommitMessage;
use crate::git::err::GitError;
use crate::Result;
use crate::{
	common::{
		conf,
		fs::{file::File, wrapper::FileLoader},
	},
	git::commit_message::GitWrapper,
};
use git2::{Repository, Signature};

use super::editmsg_status_formatter;

pub struct LibGitWrapper {
	repo: Repository,
	editmsg: Box<dyn File>,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self) -> Result<()> {
		let signature = self
			.repo
			.signature()
			.map_err(|_| GitError::LibGit("User name and/or email not set".to_string()))?;

		let commit_message = CommitMessage::from(&self.editmsg.non_empty_lines().join("\n"));

		if commit_message.has_no_content() {
			return Err(Box::new(GitError::LibGit("Commit message cannot be empty".to_string())));
		}

		self.try_to_commit(&signature, &commit_message.to_string())
			.map_err(|_| GitError::LibGit("Something went wrong!".to_string()))?;
		Ok(())
	}

	fn formatted_status(&self) -> Result<String> {
		Ok(editmsg_status_formatter::get_status_for_commit_file(&self.repo))
	}

	fn prev_commit_msg(&self) -> Result<CommitMessage> {
		let last_commit = self.repo.head().and_then(|head_ref| head_ref.peel_to_commit())?;

		Ok(CommitMessage::from(last_commit.message().unwrap_or_default()))
	}
}

impl LibGitWrapper {
	pub fn from(path: &PathBuf, file_loader: &dyn FileLoader) -> Result<Self> {
		let repo = Repository::discover(path).map_err(|_| GitError::LibGit("Could not open git repo".to_string()))?;
		let editmsg = file_loader
			.load_or_create(format!("{}/{}", path.to_string_lossy(), conf::editmsg()))
			.ok_or(GitError::Editor)?;
		if Self::no_staged_changes(&repo) {
			Err(Box::new(GitError::LibGit("No staged changes".to_string())))
		} else {
			Ok(Self { repo, editmsg })
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
