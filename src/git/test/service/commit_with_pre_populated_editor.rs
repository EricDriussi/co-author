use crate::git::{
	commit_body::{CommitBody, MockGitWrapper},
	editor::MockEditmsgEditor,
	runner::MockRunner,
	service::GitService,
	test::service::fixtures::{mock_editmsg_editor, mock_git_wrapper, mock_runner, COMMIT_MSG_HOOK, PRE_COMMIT_HOOK},
};
use mockall::{predicate::eq, Sequence};
use rstest::*;

#[rstest]
fn should_succeed(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	let msg = "a message";
	let authors = vec!["an author".to_string()];
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper
		.expect_write_to_editmsg()
		.with(eq(CommitBody::new(msg, authors.clone())))
		.returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor(msg, authors);

	assert!(result.is_ok());
}

#[rstest]
fn should_perform_actions_in_order(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	let mut seq = Sequence::new();
	mock_runner
		.expect_run()
		.times(1)
		.withf(|_, hook| hook.contains(PRE_COMMIT_HOOK))
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_write_to_editmsg()
		.times(1)
		.returning(|_| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_add_status_to_editmsg()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_editmsg_editor
		.expect_open()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	mock_runner
		.expect_run()
		.times(1)
		.withf(|_, hook| hook.contains(COMMIT_MSG_HOOK))
		.returning(|_, _| Ok(()))
		.in_sequence(&mut seq);
	mock_git_wrapper
		.expect_commit()
		.times(1)
		.returning(|| Ok(()))
		.in_sequence(&mut seq);
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(result.is_ok());
}

#[rstest]
fn should_stop_and_report_hook_error(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Err("ERR".to_string().into()));
	mock_git_wrapper.expect_write_to_editmsg().times(0);
	mock_git_wrapper.expect_add_status_to_editmsg().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_git_wrapper.expect_commit().times(0);
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(matches!(result, Err(e) if e.to_string().contains("Hook")));
}

#[rstest]
fn should_stop_when_write_to_editmsg_fails(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper
		.expect_write_to_editmsg()
		.returning(|_| Err("ERR".to_string().into()));
	mock_git_wrapper.expect_add_status_to_editmsg().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_git_wrapper.expect_commit().times(0);
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(result.is_err());
}

#[rstest]
fn should_stop_when_add_status_to_editmsg_fails(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper
		.expect_add_status_to_editmsg()
		.returning(|| Err("ERR".to_string().into()));
	mock_editmsg_editor.expect_open().times(0);
	mock_git_wrapper.expect_commit().times(0);
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(result.is_err());
}

#[rstest]
fn should_stop_when_editor_cannot_be_opened(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor
		.expect_open()
		.returning(|| Err("ERR".to_string().into()));
	mock_git_wrapper.expect_commit().times(0);
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(result.is_err());
}

#[rstest]
fn should_report_commit_error(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	let err_msg = "ERR";
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(move || Err(err_msg.into()));
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit_with_pre_populated_editor("a message", vec!["an author".to_string()]);

	assert!(matches!(result, Err(e) if e.to_string().contains(err_msg)));
}

