use crate::{
	common::fs::file_writer::MockWriter,
	git::core::{commit_message::MockGitWrapper, editor::file_editor::MockEditor, hook::MockHookRunner},
};

pub fn ok_file_writer() -> MockWriter {
	let mut mock_file = MockWriter::new();
	mock_file.expect_overwrite().returning(|_, _| Ok(()));
	mock_file.expect_append().returning(|_, _| Ok(()));
	mock_file
}

pub fn ok_hook_runner() -> MockHookRunner {
	let mut mock_hook_runner = MockHookRunner::new();
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_hook_runner
}

pub fn ok_git_wrapper(status: String) -> MockGitWrapper {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_formatted_status()
		.returning(move || Ok(status.clone()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_amend().returning(|| Ok(()));
	mock_git_wrapper
}

pub fn ok_editor() -> MockEditor {
	let mut mock_editmsg_editor = MockEditor::new();
	mock_editmsg_editor.expect_open().returning(|_| Ok(()));
	mock_editmsg_editor
}
