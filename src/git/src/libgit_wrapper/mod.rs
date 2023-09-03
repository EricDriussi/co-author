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
		match self.no_staged_changes() {
			Ok(no_changes) => {
				if no_changes {
					return Err(String::from("No changes staged for commit"));
				}
			}
			Err(_) => return Err(String::from("Something went wrong!")),
		};

		let signature = match self.repo.as_ref().unwrap().signature() {
			Ok(sig) => sig,
			Err(_) => return Err(String::from("User name and/or email not set")),
		};

		let editmsg_path = self.editmsg_path_from_root();
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
		let editmsg_path = self.editmsg_path_from_root();
		return editmsg_handler::write_commit_to_file(commit_body, editmsg_path);
	}

	fn editmsg_path_from_root(&self) -> PathBuf {
		let editmsg = ".git/COMMIT_EDITMSG";
		if let Some(mut editmsg_path) = Self::find_git_root(self.path.clone()) {
			editmsg_path.push(editmsg);
			return editmsg_path;
		} else {
			panic!("Something went wrong");
		}
	}

	fn add_status_to_editmsg(&self) -> Result<(), String> {
		let editmsg_path = self.editmsg_path_from_root();
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

	pub fn from(repo: Repository) -> Self {
		Self {
			path: repo.path().to_path_buf(),
			repo: Some(repo),
		}
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

	pub fn open_if_valid(&self) -> Option<LibGitWrapper> {
		if let Ok(repo) = Repository::open(&self.path) {
			Some(Self::from(repo))
		} else if let Some(root) = Self::find_git_root(self.path.clone()) {
			Repository::open(&root).ok().map(Self::from)
		} else {
			None
		}
	}

	fn no_staged_changes(&self) -> Result<bool, git2::Error> {
		let head = self.repo.as_ref().unwrap().head()?;
		let tree = head.peel_to_tree()?;
		let index = self.repo.as_ref().unwrap().index()?;
		let diff = self
			.repo
			.as_ref()
			.unwrap()
			.diff_tree_to_index(Some(&tree), Some(&index), None)?;
		return Ok(diff.deltas().count() == 0);
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
