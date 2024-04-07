use crate::common::fs::file_writer::MockWriter;
use crate::git::core::commit_message::CommitMessage;
use crate::git::core::editor::file_editor::MockEditor;
use crate::git::core::service::GitService;
use crate::git::core::{commit_message::MockGitWrapper, hook::MockHookRunner};

#[test]
fn return_prev_message_when_present() {
	let msg = "a message";
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Ok(CommitMessage::new(msg, vec![])));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		MockEditor::new(),
		MockWriter::new(),
	);

	let result = service.last_commit_message();

	assert_eq!(result, msg);
}

#[test]
fn return_empty_string_when_prev_message_is_not_present() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Err("ERR".into()));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		MockEditor::new(),
		MockWriter::new(),
	);

	let result = service.last_commit_message();

	assert_eq!(result, "");
}
