use crate::{
	editor_handler,
	git_domain::{CommitBody, GitWrapper},
};

pub struct GitService<T: GitWrapper> {
	git_wrapper: T,
}

impl<T: GitWrapper> GitService<T> {
	pub fn new(repo: T) -> GitService<T> {
		GitService { git_wrapper: repo }
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<(), String> {
		// TODO.better place for hooks?
		return self.git_wrapper.commit(CommitBody::new(message, authors));
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), String> {
		let editmsg_file = self.git_wrapper.setup_editmsg_file();
		// TODO.better place for hooks?
		match editor_handler::get_commit_message_from_editor(editmsg_file) {
			Some(msg) => return self.git_wrapper.commit(CommitBody::new(msg.as_str(), authors)),
			None => return Err("Commit message cannot be empty.".to_string()),
		}
	}
}
