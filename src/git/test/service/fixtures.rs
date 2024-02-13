use crate::git::{commit_body::MockGitWrapper, editor::MockEditmsgEditor, runner::MockRunner};
use rstest::*;

pub const PRE_COMMIT_HOOK: &str = "pre-commit";
pub const COMMIT_MSG_HOOK: &str = "commit-msg";

#[fixture]
pub fn mock_runner() -> MockRunner {
	MockRunner::new()
}

#[fixture]
pub fn mock_git_wrapper() -> MockGitWrapper {
	MockGitWrapper::new()
}

#[fixture]
pub fn mock_editmsg_editor() -> MockEditmsgEditor {
	MockEditmsgEditor::new()
}
