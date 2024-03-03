use crate::common::fs::file::MockFile;
use crate::git::test::service::util::{file_loader_loading, ok_editor, ok_file, ok_git_wrapper, ok_hook_runner};
use crate::git::{
	commit_message::{GitWrapper, MockGitWrapper},
	editor::{EditmsgEditor, MockEditmsgEditor},
	hook::{HookRunner, MockHookRunner},
	service::{CommitMode, GitService},
};
use crate::Result;
use mockall::predicate::eq;
use mockall::Sequence;

const ERR_MSG: &str = "an error";
const COMMIT_MSG: &str = "a message";
const AUTHOR: &str = "an author";

#[test]
fn should_succeed() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	let mut mock_file = MockFile::new();
	let mut seq = Sequence::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_file
		.expect_write()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_formatted_status()
		.times(1)
		.returning(|| Ok(String::new()))
		.in_sequence(&mut seq);
	mock_file
		.expect_write()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_editmsg_editor
		.expect_open()
		.times(1)
		.returning(|| Ok(()))
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
		&file_loader_loading(mock_file),
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[test]
fn should_write_commit_msg_and_authors() {
	let mut mock_file = MockFile::new();
	let mut seq = Sequence::new();
	mock_file
		.expect_write()
		.times(1)
		.withf(move |param| param.contains(COMMIT_MSG) && param.contains(AUTHOR))
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_file
		.expect_write()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);

	let result = do_commit(GitService::new(
		ok_git_wrapper(String::new()),
		ok_hook_runner(),
		&file_loader_loading(mock_file),
		ok_editor(),
	));

	assert!(result.is_ok());
}

#[test]
fn should_add_status_to_editmsg_file() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_file = MockFile::new();
	let mut seq = Sequence::new();
	let status = "status string";
	mock_file
		.expect_write()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_formatted_status()
		.times(1)
		.returning(move || Ok(String::from(status)))
		.in_sequence(&mut seq);
	mock_file
		.expect_write()
		.times(1)
		.with(eq(String::from(status)))
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		&file_loader_loading(mock_file),
		ok_editor(),
	));

	assert!(result.is_ok());
}

#[test]
fn should_stop_and_report_pre_commit_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	let mut mock_file = MockFile::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.returning(move || Err(ERR_MSG.into()));
	mock_file.expect_write().times(0);
	mock_git_wrapper.expect_formatted_status().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&file_loader_loading(mock_file),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_stop_and_report_add_status_to_editmsg_error() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	let mut mock_file = MockFile::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_file.expect_write().times(1).returning(|_| Ok(()));
	mock_git_wrapper
		.expect_formatted_status()
		.returning(move || Err(ERR_MSG.into()));
	mock_file.expect_write().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&file_loader_loading(mock_file),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_stop_and_report_when_editor_cannot_be_opened() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper
		.expect_formatted_status()
		.returning(|| Ok("something".to_string()));
	mock_editmsg_editor.expect_open().returning(move || Err(ERR_MSG.into()));
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&file_loader_loading(ok_file()),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_stop_and_report_commit_msg_hook_failure() {
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
		&file_loader_loading(ok_file()),
		ok_editor(),
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_report_commit_error() {
	let mut mock_git_wrapper = MockGitWrapper::new();

	mock_git_wrapper
		.expect_formatted_status()
		.returning(|| Ok("something".to_string()));
	mock_git_wrapper.expect_commit().returning(move || Err(ERR_MSG.into()));

	let result = do_commit(GitService::new(
		mock_git_wrapper,
		ok_hook_runner(),
		&file_loader_loading(ok_file()),
		ok_editor(),
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

fn do_commit<W: GitWrapper, H: HookRunner, E: EditmsgEditor>(service: Result<GitService<W, H, E>>) -> Result<()> {
	service
		.expect("could not set up git service in tests")
		.commit(CommitMode::WithEditor {
			message: Some(COMMIT_MSG),
			authors: vec![AUTHOR.to_string()],
		})
}
