use std::path::PathBuf;

use git2::{Repository, Signature, StatusOptions};

use crate::git::{CommitBody, GitRepo};

pub struct LibGitRepo {
	repo: Option<Repository>,
	path: PathBuf,
}

impl GitRepo for LibGitRepo {
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

		match self.try_to_commit(signature, commit_body) {
			Ok(_) => return Ok(()),
			Err(_) => return Err(String::from("Something went wrong!")),
		};
	}

	fn editmsg_file(&self) -> PathBuf {
		let editmsg = ".git/COMMIT_EDITMSG";
		let mut editmsg_path = Self::find_git_root(self.path.clone()).expect("Something whent wrong");
		editmsg_path.push(editmsg);
		std::fs::write(editmsg_path.clone(), self.get_status_for_commit_file()).unwrap();
		return editmsg_path;
	}
}

impl LibGitRepo {
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

	pub fn open_if_valid(&self) -> Option<LibGitRepo> {
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

	fn get_status_for_commit_file(&self) -> String {
		// TODO.refactor
		let mut options = StatusOptions::new();
		options.include_untracked(true);

		let repo = self.repo.as_ref().unwrap();
		let hd = repo.head().unwrap();
		let head = hd.shorthand();
		let statuses = repo.statuses(Some(&mut options)).unwrap();

		let mut output = String::from(
			"
# On branch ",
		);

		output.push_str(head.unwrap());
		output.push_str("\n");

		if statuses.iter().any(|e| {
			e.status().is_index_new()
				|| e.status().is_index_modified()
				|| e.status().is_index_deleted()
				|| e.status().is_index_renamed()
				|| e.status().is_index_typechange()
		}) {
			output.push_str("# Changes to be committed:\n");
			for entry in statuses.iter().filter(|e| {
				e.status().is_index_new()
					|| e.status().is_index_modified()
					|| e.status().is_index_deleted()
					|| e.status().is_index_renamed()
					|| e.status().is_index_typechange()
			}) {
				output.push_str("#\t");
				output.push_str(entry.path().unwrap());
			}
			output.push_str("\n#\n");
		}

		if statuses.iter().any(|e| e.status().is_wt_modified()) {
			output.push_str("# Changes not staged for commit:\n");
			for entry in statuses.iter().filter(|e| e.status().is_wt_modified()) {
				output.push_str("#\t");
				output.push_str(entry.path().unwrap());
			}
			output.push_str("\n#\n");
		}

		if statuses.iter().any(|e| e.status().is_wt_new()) {
			output.push_str("# Untracked files:\n");
			for entry in statuses.iter().filter(|e| e.status().is_wt_new()) {
				output.push_str("#\t");
				output.push_str(entry.path().unwrap());
			}
			output.push_str("\n");
		}

		return output;
	}
}
