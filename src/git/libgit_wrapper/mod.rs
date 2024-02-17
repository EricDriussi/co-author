use std::{error::Error, fs::OpenOptions, io::Write, path::PathBuf};

use git2::{Repository, Signature};

use crate::common::conf;

use super::commit_body::{CommitBody, GitWrapper};

pub mod editmsg_handler;

pub struct LibGitWrapper {
	repo: Repository,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self) -> Result<(), Box<dyn Error>> {
		let signature = self
			.repo
			.signature()
			.map_err(|_| "User name and/or email not set".to_string())?;

		let commit_message = editmsg_handler::read_editmsg().ok_or("Commit message cannot be empty")?;

		self.try_to_commit(&signature, &commit_message)
			.map_err(|_| "Something went wrong!".to_string())?;
		Ok(())
	}

	fn write_to_editmsg(&self, commit_body: &CommitBody) -> Result<(), Box<dyn Error>> {
		editmsg_handler::write_commit_to_file(commit_body)
	}

	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>> {
		let status = editmsg_handler::get_status_for_commit_file(&self.repo);

		let mut file_to_append = OpenOptions::new().create(true).append(true).open(conf::editmsg())?;
		file_to_append.write_all(status.as_bytes())?;
		Ok(())
	}

	fn prev_commit_msg(&self) -> Result<String, Box<dyn Error>> {
		let head_ref = self.repo.head()?;
		let last_commit = head_ref.peel_to_commit()?;

		let commit_message = last_commit.message().unwrap_or_default();
		let first_line = commit_message.lines().next().unwrap_or_default();

		Ok(first_line.to_string())
	}
}

impl LibGitWrapper {
	pub fn from(path: &PathBuf) -> Result<Self, String> {
		if let Ok(repo) = Repository::open(path) {
			return if Self::no_staged_changes(&repo) {
				Err("No staged changes".to_string())
			} else {
				Ok(Self { repo })
			};
		}
		Err("Could not open the repo".to_string())
	}

	fn no_staged_changes(repo: &Repository) -> bool {
		match repo.head() {
			Err(_) => false,
			Ok(head) => match head.peel_to_tree() {
				Err(_) => false,
				Ok(tree) => match repo.index() {
					Err(_) => false,
					Ok(index) => match repo.diff_tree_to_index(Some(&tree), Some(&index), None) {
						Err(_) => false,
						Ok(diff) => diff.deltas().count() == 0,
					},
				},
			},
		}
	}

	fn try_to_commit(&self, signature: &Signature, commit_message: &str) -> Result<(), Box<dyn Error>> {
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
