use mockall::predicate::eq;

use crate::common::conf;
use crate::common::fs::test::util::dummy_file::DummyFile;
use crate::error::assert_error_type;
use crate::git::commit_message::CommitMessage;
use crate::git::editor::simple_editor::MockEditor;
use crate::git::test::service::util::mock_helpers::{ok_file, ok_file_loader};
use crate::{
	common::fs::wrapper::MockFileLoader,
	git::{commit_message::MockGitWrapper, err::GitError, hook::MockHookRunner, service::GitService},
};

#[test]
fn get_editmsg_path_from_conf() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_or_create()
		.with(eq(conf::editmsg().clone()))
		.returning(move |_| Some(Box::new(DummyFile::empty())));

	let _ = GitService::new(
		MockGitWrapper::new(),
		MockHookRunner::new(),
		&mock_file_loader,
		MockEditor::new(),
	);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn not_instantiate_if_editmsg_is_not_present() {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader.expect_load_or_create().returning(|_| None);

	let service = GitService::new(
		MockGitWrapper::new(),
		MockHookRunner::new(),
		&mock_file_loader,
		MockEditor::new(),
	);

	assert_error_type(&service, &GitError::Editmsg);
}

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
		&ok_file_loader(ok_file()),
		MockEditor::new(),
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
		MockEditor::new(),
	)
	.expect("could not set up git service in tests");

	let result = service.last_commit_message();

	assert_eq!(result, "");
}
