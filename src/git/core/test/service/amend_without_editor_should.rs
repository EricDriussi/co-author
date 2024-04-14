use super::mock_helpers::{ok_git_wrapper, ok_hook_runner};
use crate::common::fs::file_writer::{MockWriter, Writer};
use crate::error::assert_error_contains_msg;
use crate::git::commit_mode::CommitMode;
use crate::git::core::commit_message::{GitWrapper, MockGitWrapper};
use crate::git::core::editor::file_editor::{Editor, MockEditor};
use crate::git::core::hook::{HookRunner, MockHookRunner};
use crate::git::core::service::GitService;
use crate::git::core::test::service::mock_helpers::ok_file_writer;
use crate::Result;
use mockall::Sequence;

const ERR_MSG: &str = "an error";
const COMMIT_MSG: &str = "a message";
const AUTHOR: &str = "an author";

#[test]
fn succeed() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();
	let mut seq = Sequence::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_writer
		.expect_overwrite()
		.times(1)
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_hook_runner
		.expect_run_commit_msg()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_amend()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		MockEditor::new(),
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn not_create_new_commit() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper.expect_amend().times(1).returning(|| Ok(()));
	mock_git_wrapper.expect_commit().times(0);

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		MockEditor::new(),
		ok_file_writer(),
	));

	assert!(result.is_ok());
}

#[test]
fn write_commit_msg_and_authors() {
	let mut mock_writer = MockWriter::new();
	mock_writer
		.expect_overwrite()
		.withf(move |_, param| param.contains(COMMIT_MSG) && param.contains(AUTHOR))
		.returning(|_, _| Ok(()));

	let result = do_amend(GitService::new(
		ok_git_wrapper(String::new()),
		ok_hook_runner(),
		MockEditor::new(),
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn not_add_status_to_editmsg_file() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();
	mock_writer.expect_overwrite().times(1).returning(|_, _| Ok(()));
	mock_git_wrapper.expect_formatted_status().times(0);
	mock_writer.expect_append().times(0);
	mock_git_wrapper.expect_amend().returning(|| Ok(()));

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		MockEditor::new(),
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn not_open_editor() {
	let mut mock_editor = MockEditor::new();
	let mut mock_writer = MockWriter::new();
	mock_writer.expect_overwrite().returning(|_, _| Ok(()));
	mock_editor.expect_open().times(0);

	let result = do_amend(GitService::new(
		ok_git_wrapper(String::new()),
		ok_hook_runner(),
		mock_editor,
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn stop_and_report_pre_commit_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.returning(move || Err(ERR_MSG.into()));
	mock_writer.expect_overwrite().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_amend().times(0);

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		MockEditor::new(),
		mock_writer,
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn stop_and_report_commit_msg_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_writer.expect_overwrite().returning(|_, _| Ok(()));
	mock_hook_runner
		.expect_run_commit_msg()
		.returning(move || Err(ERR_MSG.into()));
	mock_git_wrapper.expect_amend().times(0);

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		MockEditor::new(),
		mock_writer,
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn report_amend_error() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();
	mock_writer.expect_overwrite().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_amend().returning(move || Err(ERR_MSG.into()));

	let result = do_amend(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		MockEditor::new(),
		mock_writer,
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

fn do_amend<G: GitWrapper, H: HookRunner, E: Editor, W: Writer>(mut service: GitService<G, H, E, W>) -> Result<()> {
	service.commit(CommitMode::WithoutEditor {
		message: COMMIT_MSG,
		authors: vec![AUTHOR.to_string()],
		amend: true,
	})
}
