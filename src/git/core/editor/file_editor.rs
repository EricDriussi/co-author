use crate::{
	common::runner::Runner,
	git::{core::conf_provider::ConfProvider, err::GitError},
	Result,
};
use std::env;

#[cfg_attr(test, mockall::automock)]
pub trait Editor {
	fn open(&self, editmsg: &str) -> Result<()>;
}

pub struct FileEditor<R: Runner, C: ConfProvider> {
	runner: R,
	conf_provider: C,
}

impl<R: Runner, C: ConfProvider> Editor for FileEditor<R, C> {
	fn open(&self, editmsg: &str) -> Result<()> {
		let editing_operation_result = match self.conf_provider.get_editor() {
			None => self.env_fallback(editmsg),
			Some(git_editor) => self.runner.spawn(&git_editor, editmsg),
		};

		Ok(editing_operation_result.map_err(|_| GitError::Editor)?)
	}
}

impl<R: Runner, C: ConfProvider> FileEditor<R, C> {
	pub fn new(runner: R, conf_provider: C) -> Self {
		Self { runner, conf_provider }
	}

	fn env_fallback(&self, path: &str) -> Result<()> {
		match env::var("EDITOR") {
			Err(_) => self.vim_fallback(path),
			Ok(editor) => self.runner.spawn(&editor, path),
		}
	}

	fn vim_fallback(&self, path: &str) -> Result<()> {
		self.runner
			.spawn("vim", path)
			.or_else(|_| self.runner.spawn("vi", path))
	}
}
