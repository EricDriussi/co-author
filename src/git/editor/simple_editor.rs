use super::conf_provider::DefaultEditorGetter;
use crate::{
	common::{fs::file::File, runner::Runner},
	git::err::GitError,
	Result,
};
use std::env;

#[cfg_attr(test, mockall::automock)]
pub trait Editor {
	fn open(&self, editmsg: &dyn File) -> Result<()>;
}

pub struct SimpleEditor<R: Runner, C: DefaultEditorGetter> {
	runner: R,
	conf_provider: C,
}

impl<R: Runner, C: DefaultEditorGetter> Editor for SimpleEditor<R, C> {
	fn open(&self, editmsg: &dyn File) -> Result<()> {
		let editing_operation_result = match self.conf_provider.get_editor() {
			None => self.env_fallback(editmsg.path()),
			Some(git_editor) => self.runner.spawn(&git_editor, editmsg.path()),
		};

		Ok(editing_operation_result.map_err(|_| GitError::Editor)?)
	}
}

impl<R: Runner, C: DefaultEditorGetter> SimpleEditor<R, C> {
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
