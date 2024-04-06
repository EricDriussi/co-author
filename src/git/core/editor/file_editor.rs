use crate::{
	common::{env, runner::Runner},
	git::{core::conf_provider::ConfProvider, err::GitError},
	Result,
};

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
		match self.conf_provider.get_editor() {
			None => self.env_fallback(editmsg),
			Some(git_editor) => Ok(self.runner.spawn(&git_editor, editmsg).map_err(|_| GitError::Editor)?),
		}
	}
}

impl<R: Runner, C: ConfProvider> FileEditor<R, C> {
	pub fn new(runner: R, conf_provider: C) -> Self {
		Self { runner, conf_provider }
	}

	fn env_fallback(&self, path: &str) -> Result<()> {
		match env::editor() {
			Err(_) => self.vim_fallback(path),
			Ok(editor) => Ok(self.runner.spawn(&editor, path).map_err(|_| GitError::Editor)?),
		}
	}

	fn vim_fallback(&self, path: &str) -> Result<()> {
		Ok(self
			.runner
			.spawn("vim", path)
			.or_else(|_| self.runner.spawn("vi", path))
			.map_err(|_| GitError::Editor)?)
	}
}
