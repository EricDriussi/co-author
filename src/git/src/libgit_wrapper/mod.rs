use std::{error::Error, fs::OpenOptions, io::Write, path::PathBuf};

use git2::{Repository, Signature};

use crate::git::{CommitBody, GitWrapper};

pub mod editmsg_handler;

pub struct LibGitWrapper {
	repo: Option<Repository>,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self) -> Result<(), Box<dyn Error>> {
		let signature = self
			.repo
			.as_ref()
			.unwrap()
			.signature()
			.map_err(|_| "User name and/or email not set".to_string())?;

		let commit_message = editmsg_handler::read_editmsg().ok_or("Commit message cannot be empty".to_string())?;

		self.try_to_commit(signature, commit_message)
			.map_err(|_| "Something went wrong!".to_string())?;
		Ok(())
	}

	fn write_to_editmsg(&self, commit_body: CommitBody) -> Result<(), Box<dyn Error>> {
		return editmsg_handler::write_commit_to_file(commit_body);
	}

	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>> {
		let status = editmsg_handler::get_status_for_commit_file(&self.repo.as_ref().unwrap());

		let mut file_to_append = OpenOptions::new().create(true).append(true).open(conf::editmsg())?;
		file_to_append.write_all(status.as_bytes())?;
		Ok(())
	}
}

impl LibGitWrapper {
	pub fn from(path: PathBuf) -> Result<Self, String> {
		if let Ok(repo) = Repository::open(path.clone()) {
			return match Self::no_staged_changes(&repo) {
				true => Err("No staged changes".to_string()),
				false => Ok(Self { repo: Some(repo) }),
			};
		}
		return Err("Could not open the repo".to_string());
	}

	fn no_staged_changes(repo: &Repository) -> bool {
		let head = repo.head().unwrap();
		let tree = head.peel_to_tree().unwrap();
		let index = repo.index().unwrap();
		let diff = repo.diff_tree_to_index(Some(&tree), Some(&index), None).unwrap();
		return diff.deltas().count() == 0;
	}

	fn try_to_commit(&self, signature: Signature, commit_message: String) -> Result<(), git2::Error> {
		let oid = self.repo.as_ref().unwrap().index()?.write_tree()?;
		let tree = self.repo.as_ref().unwrap().find_tree(oid)?;
		let parent_commit = self.repo.as_ref().unwrap().head()?.peel_to_commit()?;
		self.repo
			.as_ref()
			.unwrap()
			.commit(
				Some("HEAD"),
				&signature,
				&signature,
				commit_message.as_str(),
				&tree,
				&[&parent_commit],
			)
			.map(|_| ())
	}
}
