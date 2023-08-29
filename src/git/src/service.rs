use crate::{
	editor_handler,
	git_domain::{CommitBody, GitWrapper},
};

pub struct GitService<T: GitWrapper> {
	repo: T,
}

impl<T: GitWrapper> GitService<T> {
	pub fn new(repo: T) -> GitService<T> {
		GitService { repo }
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), String> {
		return self.repo.commit(CommitBody::new(message, authors));
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), String> {
		let editmsg = self.repo.editmsg_file();
		match editor_handler::get_commit_message_from_editor(editmsg) {
			Some(msg) => return self.repo.commit(CommitBody::new(msg.as_str(), authors)),
			None => return Err("Commit message cannot be empty.".to_string()),
		}
	}
}
