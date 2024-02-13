use crate::conf;
use crate::Result;

use super::{
	commit_body::{CommitBody, GitWrapper},
	editor::EditmsgEditor,
	git_err::GitError,
	runner::Runner,
};

pub struct GitService<W: GitWrapper, R: Runner, E: EditmsgEditor> {
	git_wrapper: W,
	runner: R,
	editmsg_editor: E,
	hook_shell: &'static str,
}

impl<W: GitWrapper, R: Runner, E: EditmsgEditor> GitService<W, R, E> {
	pub fn new(git_wrapper: W, runner: R, editmsg_editor: E) -> Self {
		Self {
			git_wrapper,
			runner,
			editmsg_editor,
			hook_shell: "sh",
		}
	}

	pub fn last_commit_message(&self) -> String {
		self.git_wrapper.prev_commit_msg().unwrap_or_default()
	}

	pub fn commit(&self, message: &str, authors: Vec<String>) -> Result<()> {
		self.run_hook(&Hook::PreCommit)?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new(message, authors))?;
		self.run_hook(&Hook::CommitMsg)?;
		self.git_wrapper.commit()
	}

	pub fn commit_with_editor(&self, authors: Vec<String>) -> Result<()> {
		self.run_hook(&Hook::PreCommit)?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new("", authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		self.editmsg_editor.open()?;
		self.run_hook(&Hook::CommitMsg)?;
		self.git_wrapper.commit()
	}

	pub fn commit_with_pre_populated_editor(&self, message: &str, authors: Vec<String>) -> Result<()> {
		self.run_hook(&Hook::PreCommit)?;
		self.git_wrapper.write_to_editmsg(&CommitBody::new(message, authors))?;
		self.git_wrapper.add_status_to_editmsg()?;
		self.editmsg_editor.open()?;
		self.run_hook(&Hook::CommitMsg)?;
		self.git_wrapper.commit()
	}

	fn run_hook(&self, hook: &Hook) -> Result<()> {
		let hook_name = match hook {
			Hook::PreCommit => "pre-commit",
			Hook::CommitMsg => "commit-msg",
		};

		let hook_path = format!("{}/{}", conf::hooks_path(), hook_name);

		Ok(self
			.runner
			.run(self.hook_shell, hook_path.as_str())
			.map_err(|_| GitError::Hook(hook_name.to_string()))?)
	}
}

enum Hook {
	PreCommit,
	CommitMsg,
}
