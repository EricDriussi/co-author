use super::conf_provider::ConfProvider;
use super::git_err::GitError;
use crate::{
	common::{conf, fs::wrapper::FileLoader, runner::Runner},
	Result,
};
use std::env;

#[cfg_attr(test, mockall::automock)]
pub trait EditmsgEditor {
	fn open(&self) -> Result<()>;
}

pub struct Editor<R: Runner, F: FileLoader, C: ConfProvider> {
	runner: R,
	file_loader: F, // TODO: this should not be here
	conf_provider: C,
}

impl<R: Runner, F: FileLoader, C: ConfProvider> EditmsgEditor for Editor<R, F, C> {
	// TODO: this should take a File as param
	fn open(&self) -> Result<()> {
		let editmsg = self.file_loader.load(conf::editmsg()).ok_or(GitError::Editor)?;

		let editing_operation_result = match self.conf_provider.get_editor() {
			None => self.env_fallback(editmsg.path()),
			Some(git_editor) => self.runner.spawn(&git_editor, editmsg.path()),
		};

		Ok(editing_operation_result.map_err(|_| GitError::Editor)?)
	}
}

impl<R: Runner, F: FileLoader, C: ConfProvider> Editor<R, F, C> {
	pub fn new(runner: R, file_loader: F, conf_provider: C) -> Self {
		Self {
			runner,
			file_loader,
			conf_provider,
		}
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
