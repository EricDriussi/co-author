use std::path::PathBuf;

use git2::{Repository, Signature};

use crate::git_domain::{CommitBody, GitWrapper};

mod editmsg_handler;

pub struct LibGitWrapper {
	repo: Option<Repository>,
	path: PathBuf,
}

impl GitWrapper for LibGitWrapper {
	fn commit(&self, commit_body: CommitBody) -> Result<(), String> {
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

		// TODO: run pre-commit hook
		// TODO: run prepare-commit-msg hook (this only makes sense when using the editor, run elsewhere?)
		// TODO: run commit-msg hook (passing the name of the editmsg file is the default git behavior, maybe print to editmsg even if -m or prompt?)

		match self.try_to_commit(signature, commit_body) {
			Ok(_) => return Ok(()),
			Err(_) => return Err(String::from("Something went wrong!")),
		};
	}

	fn setup_editmsg_file(&self) -> PathBuf {
		let editmsg = ".git/COMMIT_EDITMSG";
		let mut editmsg_path = Self::find_git_root(self.path.clone()).expect("Something whent wrong");
		editmsg_path.push(editmsg);
		std::fs::write(
			editmsg_path.clone(),
			editmsg_handler::get_status_for_commit_file(self.repo.as_ref().unwrap()),
		)
		.unwrap();
		return editmsg_path;
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

	fn try_to_commit(&self, signature: Signature, commit_body: CommitBody) -> Result<(), git2::Error> {
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
				&commit_body.formatted_body(),
				&tree,
				&[&parent_commit],
			)
			.map(|_| ())
	}
}
