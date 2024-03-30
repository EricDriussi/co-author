use crate::{
	common::{conf, runner::Runner},
	git::err::GitError,
	Result,
};
use std::path::Path;

#[cfg_attr(test, mockall::automock)]
pub trait HookRunner {
	fn run_pre_commit(&self) -> Result<()>;
	fn run_commit_msg(&self) -> Result<()>;
}

pub struct Hook<R: Runner> {
	runner: R,
	shell: &'static str,
	path: String,
	pre_commit: &'static str,
	commit_msg: &'static str,
}

impl<R: Runner> Hook<R> {
	pub fn new(runner: R) -> Self {
		Self {
			runner,
			shell: "sh",
			path: conf::hooks_path(),
			pre_commit: "pre-commit",
			commit_msg: "commit-msg",
		}
	}

	fn run_hook(&self, hook: &str) -> Result<()> {
		let hook_path = format!("{}/{}", self.path, hook);
		if !Path::new(&hook_path).exists() {
			return Ok(());
		}
		Ok(self
			.runner
			.run(self.shell, hook_path.as_str())
			.map_err(|_| GitError::Hook(hook.to_string()))?)
	}
}

impl<R: Runner> HookRunner for Hook<R> {
	fn run_pre_commit(&self) -> Result<()> {
		self.run_hook(self.pre_commit)
	}

	fn run_commit_msg(&self) -> Result<()> {
		self.run_hook(self.commit_msg)
	}
}
