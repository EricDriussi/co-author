use super::mock_helpers::{ok_editor, ok_file_writer, ok_git_wrapper, ok_hook_runner};
use crate::common::fs::file_writer::{MockWriter, Writer};
use crate::error::assert_error_contains_msg;
use crate::git::commit_mode::CommitMode;
use crate::git::core::commit_message::{GitWrapper, MockGitWrapper};
use crate::git::core::editor::file_editor::{Editor, MockEditor};
use crate::git::core::hook::{HookRunner, MockHookRunner};
use crate::git::core::service::GitService;
use crate::Result;
use mockall::predicate::{always, eq};
use mockall::Sequence;

const ERR_MSG: &str = "an error";
const COMMIT_MSG: &str = "a message";
const AUTHOR: &str = "an author";

#[test]
fn succeed() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editor = MockEditor::new();
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
	mock_git_wrapper
		.expect_formatted_status()
		.times(1)
		.returning(|| Ok(String::new()))
		.in_sequence(&mut seq);
	mock_writer
		.expect_append()
		.times(1)
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_editor
		.expect_open()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_hook_runner
		.expect_run_commit_msg()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_commit()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editor,
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn not_amend() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_formatted_status()
		.times(1)
		.returning(|| Ok(String::new()));
	mock_git_wrapper.expect_commit().times(1).returning(|| Ok(()));
	mock_git_wrapper.expect_amend().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		ok_editor(),
		ok_file_writer(),
	));

	assert!(result.is_ok());
}

#[test]
fn write_commit_msg_and_authors() {
	let mut mock_writer = MockWriter::new();
	let mut seq = Sequence::new();
	mock_writer
		.expect_overwrite()
		.times(1)
		.withf(move |_, param| param.contains(COMMIT_MSG) && param.contains(AUTHOR))
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_writer
		.expect_append()
		.times(1)
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);

	let result = do_commit(GitService::new(
		ok_git_wrapper(String::new()),
		ok_hook_runner(),
		ok_editor(),
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn add_status_to_editmsg_file() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_writer = MockWriter::new();
	let mut seq = Sequence::new();
	let status = "status string";
	mock_writer
		.expect_overwrite()
		.times(1)
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_formatted_status()
		.times(1)
		.returning(move || Ok(String::from(status)))
		.in_sequence(&mut seq);
	mock_writer
		.expect_append()
		.times(1)
		.with(always(), eq(String::from(status)))
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		ok_editor(),
		mock_writer,
	));

	assert!(result.is_ok());
}

#[test]
fn stop_and_report_pre_commit_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editor = MockEditor::new();
	let mut mock_writer = MockWriter::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.returning(move || Err(ERR_MSG.into()));
	mock_writer.expect_overwrite().times(0);
	mock_git_wrapper.expect_formatted_status().times(0);
	mock_writer.expect_append().times(0);
	mock_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editor,
		mock_writer,
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn stop_and_report_add_status_to_editmsg_error() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editor = MockEditor::new();
	let mut mock_writer = MockWriter::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_writer.expect_overwrite().times(1).returning(|_, _| Ok(()));
	mock_git_wrapper
		.expect_formatted_status()
		.returning(move || Err(ERR_MSG.into()));
	mock_writer.expect_append().times(0);
	mock_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editor,
		mock_writer,
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn stop_and_report_when_editor_cannot_be_opened() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editor = MockEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper
		.expect_formatted_status()
		.returning(|| Ok("something".to_string()));
	mock_editor.expect_open().returning(move |_| Err(ERR_MSG.into()));
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editor,
		ok_file_writer(),
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn stop_and_report_commit_msg_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper
		.expect_formatted_status()
		.returning(|| Ok("something".to_string()));
	mock_hook_runner
		.expect_run_commit_msg()
		.returning(move || Err(ERR_MSG.into()));
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		ok_editor(),
		ok_file_writer(),
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

#[test]
fn report_commit_error() {
	let mut mock_git_wrapper = MockGitWrapper::new();

	mock_git_wrapper
		.expect_formatted_status()
		.returning(|| Ok("something".to_string()));
	mock_git_wrapper.expect_commit().returning(move || Err(ERR_MSG.into()));

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		ok_editor(),
		ok_file_writer(),
	));

	assert_error_contains_msg(&result, ERR_MSG);
}

fn do_commit<G: GitWrapper, H: HookRunner, E: Editor, W: Writer>(mut service: GitService<G, H, E, W>) -> Result<()> {
	service.commit(CommitMode::WithEditor {
		message: Some(COMMIT_MSG),
		authors: vec![AUTHOR.to_string()],
		amend: false,
	})
}
