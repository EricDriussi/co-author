use crate::common::runner::MockRunner;
use crate::git::{commit_body::MockGitWrapper, editor::MockEditmsgEditor, hook::MockHookRunner};
use rstest::*;

#[fixture]
pub fn mock_runner() -> MockRunner {
	MockRunner::new()
}

#[fixture]
pub fn mock_hook_runner() -> MockHookRunner {
	MockHookRunner::new()
}

#[fixture]
pub fn mock_git_wrapper() -> MockGitWrapper {
	MockGitWrapper::new()
}

#[fixture]
pub fn mock_editmsg_editor() -> MockEditmsgEditor {
	MockEditmsgEditor::new()
}
