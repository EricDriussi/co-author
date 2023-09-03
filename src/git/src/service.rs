use crate::{
	editor,
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
		// TODO. pre-commit hook
		self.git_wrapper.write_to_editmsg(CommitBody::new(message, authors))?;
		// TODO. commit-msg hook (pass editmsg path as param)
		return self.git_wrapper.commit();
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<(), String> {
		// TODO. pre-commit hook
		self.git_wrapper.write_to_editmsg(CommitBody::new("", authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		let editmsg_path = self.git_wrapper.editmsg_path_from_root();
		editor::open(editmsg_path);
		// TODO. commit-msg hook (pass editmsg path as param)
		return self.git_wrapper.commit();
	}
}
