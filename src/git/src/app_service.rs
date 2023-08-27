use crate::{
	editor,
	git::{CommitBody, GitRepo},
};

pub struct GitService<T: GitRepo> {
	repo: T,
}

impl<T: GitRepo> GitService<T> {
	pub fn new(repo: T) -> GitService<T> {
		GitService { repo }
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), String> {
		return self.repo.commit(CommitBody::new(message, authors));
	}

	pub fn commit_with_editor(&self, authors: Vec<String>, tmp_file: String) -> Result<(), String> {
		match editor::get_commit_message_from_editor(tmp_file) {
			Some(msg) => return self.repo.commit(CommitBody::new(msg.as_str(), authors)),
			None => return Err("Commit message cannot be empty.".to_string()),
		}
	}
}
