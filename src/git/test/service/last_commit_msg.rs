use crate::git::{
	commit_message::MockGitWrapper,
	editor::MockEditmsgEditor,
	hook::MockHookRunner,
	service::GitService,
	test::service::util::{file_loader_loading, ok_file},
};

#[test]
fn should_return_message_when_present() {
	let msg = "a message";
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Ok(msg.to_string()));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		&file_loader_loading(ok_file()),
		MockEditmsgEditor::new(),
	);

	let result = service.last_commit_message();

	assert_eq!(result, msg);
}

#[test]
fn should_return_empty_string_when_message_is_not_present() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Err("ERR".to_string().into()));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		&file_loader_loading(ok_file()),
		MockEditmsgEditor::new(),
	);

	let result = service.last_commit_message();

	assert_eq!(result, "");
}
