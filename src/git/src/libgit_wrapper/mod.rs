use std::{fs::OpenOptions, io::Write, path::PathBuf};

use git2::{Repository, Signature};

use crate::git_domain::{CommitBody, GitWrapper};

pub mod editmsg_handler;

pub struct LibGitWrapper {
	repo: Option<Repository>,
	path: PathBuf,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self) -> Result<(), String> {
		let signature = match self.repo.as_ref().unwrap().signature() {
			Ok(sig) => sig,
			Err(_) => return Err(String::from("User name and/or email not set")),
		};

		let commit_message = match editmsg_handler::read_editmsg(&self.editmsg_path()) {
			Some(msg) => msg,
			None => return Err(String::from("Commit message cannot be empty")),
		};

		match self.try_to_commit(signature, commit_message) {
			Ok(_) => return Ok(()),
			Err(_) => return Err(String::from("Something went wrong!")),
		};
	}

	fn write_to_editmsg(&self, commit_body: CommitBody) -> Result<(), String> {
		return editmsg_handler::write_commit_to_file(commit_body, self.editmsg_path());
	}

	fn editmsg_path(&self) -> PathBuf {
		return self.path.join(".git/COMMIT_EDITMSG");
	}

	fn hooks_path(&self) -> PathBuf {
		return self.path.join(".git/hooks/");
	}

	fn add_status_to_editmsg(&self) -> Result<(), String> {
		let editmsg_path = self.editmsg_path();
		let status = editmsg_handler::get_status_for_commit_file(&self.repo.as_ref().unwrap());

		let mut file_to_append = OpenOptions::new().create(true).append(true).open(editmsg_path).unwrap();
		match file_to_append.write_all(status.as_bytes()) {
			Ok(_) => Ok(()),
			Err(_) => Err("Couldn't write status".to_string()),
		}
	}
}

impl LibGitWrapper {
	pub fn new(path: PathBuf) -> Self {
		Self { path, repo: None }
	}

	pub fn from(path: PathBuf) -> Result<Self, String> {
		if let Ok(repo) = Repository::open(path.clone()) {
			return match Self::no_staged_changes(&repo) {
				true => Err("No staged changes".to_string()),
				false => Ok(Self { path, repo: Some(repo) }),
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
