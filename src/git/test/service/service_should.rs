use crate::common::file_writer::MockWriter;
use crate::git::commit_message::CommitMessage;
use crate::git::editor::simple_editor::MockEditor;
use crate::git::{commit_message::MockGitWrapper, hook::MockHookRunner, service::GitService};

#[test]
fn return_message_when_present() {
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
fn return_empty_string_when_message_is_not_present() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Err("ERR".to_string().into()));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		MockEditor::new(),
		MockWriter::new(),
	);

	let result = service.last_commit_message();

	assert_eq!(result, "");
}
