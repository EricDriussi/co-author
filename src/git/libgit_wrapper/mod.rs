use super::{commit_message::GitWrapper, err::GitError};
use crate::common::{
	conf,
	fs::{file::File, wrapper::FileLoader},
};
use crate::Result;
use git2::{Repository, Signature};
pub mod editmsg_handler;

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

		let commit_message = self
			.editmsg
			.non_empty_lines()
			.into_iter()
			.filter(|line| !line.starts_with('#'))
			.collect::<Vec<String>>()
			.join("\n");

		let commit_only_has_co_author_lines = commit_message
			.split('\n')
			.filter(|line| !line.starts_with(&conf::co_author_prefix()))
			.collect::<Vec<&str>>()
			.join("\n")
			.is_empty();

		if commit_only_has_co_author_lines {
			return Err(Box::new(GitError::LibGit("Commit message cannot be empty".to_string())));
		}

		self.try_to_commit(&signature, &commit_message)
			.map_err(|_| GitError::LibGit("Something went wrong!".to_string()))?;
		Ok(())
	}

	fn formatted_status(&self) -> Result<String> {
		Ok(editmsg_handler::get_status_for_commit_file(&self.repo))
	}

	fn prev_commit_msg(&self) -> Result<String> {
		let last_commit = self.repo.head().and_then(|head_ref| head_ref.peel_to_commit())?;

		let first_line = last_commit
			.message()
			.and_then(|msg| msg.lines().next())
			.unwrap_or_default();

		Ok(first_line.to_string())
	}
}

impl LibGitWrapper {
	pub fn from(path: &str, file_loader: &dyn FileLoader) -> Result<Self> {
		let repo = Repository::open(path).map_err(|_| GitError::LibGit("Could not open git repo".to_string()))?;
		let editmsg = file_loader
			.load_or_create(format!("{path}/{}", conf::editmsg()))
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
