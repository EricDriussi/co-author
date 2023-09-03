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

		let editmsg_path = self.editmsg_path();
		let commit_message = match editmsg_handler::read_editmsg(&editmsg_path) {
			Some(msg) => msg,
			None => return Err(String::from("Commit message cannot be empty")),
		};

		match self.try_to_commit(signature, commit_message) {
			Ok(_) => return Ok(()),
			Err(_) => return Err(String::from("Something went wrong!")),
		};
	}

	fn write_to_editmsg(&self, commit_body: CommitBody) -> Result<(), String> {
		let editmsg_path = self.editmsg_path();
		return editmsg_handler::write_commit_to_file(commit_body, editmsg_path);
	}

	fn editmsg_path(&self) -> PathBuf {
		let editmsg = ".git/COMMIT_EDITMSG";
		if let Some(mut editmsg_path) = Self::find_git_root(self.path.clone()) {
			editmsg_path.push(editmsg);
			return editmsg_path;
		} else {
			panic!("Something went wrong");
		}
	}

	fn hooks_path(&self) -> PathBuf {
		if let Some(mut hooks_path) = Self::find_git_root(self.path.clone()) {
			hooks_path.push(".git/hooks/");
			return hooks_path;
		} else {
			panic!("Something went wrong");
		}
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
		let root = match Self::find_git_root(path.clone()) {
			Some(root) => root,
			None => return Err("Not in a valid git repo".to_string()),
		};

		if let Ok(repo) = Repository::open(root) {
			return match Self::no_staged_changes(&repo) {
				true => Err("No staged changes".to_string()),
				false => Ok(Self { path, repo: Some(repo) }),
			};
		}
		return Err("Could not open the repo".to_string());
	}

	fn find_git_root(mut path: PathBuf) -> Option<PathBuf> {
		loop {
			let git_dir = path.join(".git");
			if git_dir.is_dir() {
				return Some(path);
			}

			if !path.pop() {
				break;
			}
		}
		None
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
