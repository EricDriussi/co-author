use crate::error::assert_error_type;
use crate::git::test::service::util::mock_helpers::{ok_file, ok_file_loader};
use crate::{
	common::fs::wrapper::MockFileLoader,
	git::{
		commit_message::MockGitWrapper, editor::MockEditmsgEditor, err::GitError, hook::MockHookRunner,
		service::GitService,
	},
};

#[test]
fn not_instantiate_if_editmsg_is_not_present() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_or_create().returning(|_| None);

	let service = GitService::new(
		MockGitWrapper::new(),
		MockHookRunner::new(),
		&mock_file_loader,
		MockEditmsgEditor::new(),
	);

	assert_error_type(&service, &GitError::Editmsg);
}

#[test]
fn return_message_when_present() {
	let msg = "a message";
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_prev_commit_msg()
		.returning(|| Ok(msg.to_string()));
	let service = GitService::new(
		mock_git_wrapper,
		MockHookRunner::new(),
		&ok_file_loader(ok_file()),
		MockEditmsgEditor::new(),
	)
	.expect("could not set up git service in tests");

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
		&ok_file_loader(ok_file()),
		MockEditmsgEditor::new(),
	)
	.expect("could not set up git service in tests");

	let result = service.last_commit_message();

	assert_eq!(result, "");
}
