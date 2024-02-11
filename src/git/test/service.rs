use crate::git::{commit_body::MockGitWrapper, editor::MockEditmsgEditor, runner::MockRunner, service::GitService};

#[test]
fn should_return_last_commit_message_when_present() {
	let msg = "a message";
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Ok(msg.to_string()));
	let service = GitService::new(mock_git_wrapper, MockRunner::new(), MockEditmsgEditor::new());

	let result = service.last_commit_message();

	assert_eq!(result, msg);
}

#[test]
fn should_return_empty_string_when_last_commit_message_is_not_present() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Err("ERR".to_string().into()));
	let service = GitService::new(mock_git_wrapper, MockRunner::new(), MockEditmsgEditor::new());

	let result = service.last_commit_message();

	assert_eq!(result, "");
}

#[test]
fn should_commit() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("pre-commit"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("commit-msg"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, MockEditmsgEditor::new());

	let result = service.commit("a message", vec!["an author".to_string()]);

	assert!(result.is_ok());
}

#[test]
fn should_commit_with_editor() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_runner = MockRunner::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("pre-commit"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("commit-msg"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_editor(vec!["an author".to_string()]);

	assert!(result.is_ok());
}

#[test]
fn should_commit_with_pre_populated_editor() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_runner = MockRunner::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("pre-commit"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("commit-msg"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(result.is_ok());
}
