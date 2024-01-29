use std::env;

use crate::Result;
use crate::{conf, fs::wrapper::FileLoader};

use super::conf_provider::ConfProvider;
use super::git_err::GitError;
use super::runner::Runner;

pub struct TextEditor<R: Runner, F: FileLoader, C: ConfProvider> {
	runner: R,
	file_loader: F,
	conf_provider: C,
}

impl<R: Runner, F: FileLoader, C: ConfProvider> TextEditor<R, F, C> {
	pub fn new(runner: R, file_loader: F, conf_provider: C) -> Self {
		Self {
			runner,
			file_loader,
			conf_provider,
		}
	}

	pub fn open_editmsg(&self) -> Result<()> {
		let editmsg = self
			.file_loader
			.load_creating(conf::editmsg())
			.ok_or(GitError::Editor)?;

		match self.conf_provider.get_editor() {
			None => self.env_fallback(editmsg.path()),
			Some(git_editor) => self.runner.open_editor(&git_editor, editmsg.path()),
		}
	}

	fn env_fallback(&self, path: &str) -> Result<()> {
		match env::var("EDITOR") {
			Err(_) => self.vim_fallback(path),
			Ok(editor) => self.runner.open_editor(&editor, path),
		}
	}

	fn vim_fallback(&self, path: &str) -> Result<()> {
		self.runner
			.open_editor("vim", path)
			.or_else(|_| self.runner.open_editor("vi", path))
	}
}
