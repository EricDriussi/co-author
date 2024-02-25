use crate::git::{
	commit_message::{GitWrapper, MockGitWrapper},
	editor::{EditmsgEditor, MockEditmsgEditor},
	hook::{HookRunner, MockHookRunner},
	service::{CommitMode, GitService},
	test::service::util::successful_file_loader,
};
use crate::Result;
use mockall::Sequence;

const ERR_MSG: &str = "ERR";

#[test]
fn should_succeed() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[test]
fn should_write_commit_msg_and_authors() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	let message = "a message";
	let authors = vec!["an author".to_string()];
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let mut service = GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	);

	let result = service.commit(CommitMode::WithoutEditor { message, authors });

	assert!(result.is_ok());
}

#[test]
fn should_not_add_status_to_editmsg() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[test]
fn should_not_open_editor() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().times(0);

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[test]
fn should_perform_actions_in_order() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	let mut seq = Sequence::new();
	mock_hook_runner
		.expect_run_pre_commit()
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

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[test]
fn should_stop_and_report_pre_commit_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner
		.expect_run_pre_commit()
		.returning(move || Err(ERR_MSG.into()));
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_stop_and_report_commit_msg_hook_failure() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner
		.expect_run_commit_msg()
		.returning(move || Err(ERR_MSG.into()));
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[test]
fn should_report_commit_error() {
	let mut mock_hook_runner = MockHookRunner::new();
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mock_editmsg_editor = MockEditmsgEditor::new();

	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(move || Err(ERR_MSG.into()));

	let result = do_commit(&mut GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		&successful_file_loader(),
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

fn do_commit<W: GitWrapper, H: HookRunner, E: EditmsgEditor>(service: &mut GitService<W, H, E>) -> Result<()> {
	service.commit(CommitMode::WithoutEditor {
		message: "a message",
		authors: vec!["an author".to_string()],
	})
}
