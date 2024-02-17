use crate::git::{
	commit_body::{CommitBody, GitWrapper, MockGitWrapper},
	editor::{EditmsgEditor, MockEditmsgEditor},
	hook::{HookRunner, MockHookRunner},
	service::{CommitMode, GitService},
	test::fixtures::{mock_editmsg_editor, mock_git_wrapper, mock_hook_runner},
};
use crate::Result;
use mockall::{predicate::eq, Sequence};
use rstest::*;

const ERR_MSG: &str = "ERR";

#[rstest]
fn should_succeed(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[rstest]
fn should_write_commit_msg_and_authors(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	let message = "a message";
	let authors = vec!["an author".to_string()];
	mock_git_wrapper
		.expect_write_to_editmsg()
		.with(eq(CommitBody::new(message, authors.clone())))
		.returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_hook_runner, mock_editmsg_editor);

	let result = service.commit(CommitMode::WithoutEditor { message, authors });

	assert!(result.is_ok());
}

#[rstest]
fn should_perform_actions_in_order(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	let mut seq = Sequence::new();
	mock_hook_runner
		.expect_run_pre_commit()
		.times(1)
		.returning(|| Ok(()))
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

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(result.is_ok());
}

#[rstest]
fn should_stop_and_report_pre_commit_hook_failure(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner
		.expect_run_pre_commit()
		.returning(move || Err(ERR_MSG.into()));
	mock_git_wrapper.expect_write_to_editmsg().times(0);
	mock_git_wrapper.expect_add_status_to_editmsg().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_stop_and_report_write_to_editmsg_error(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper
		.expect_write_to_editmsg()
		.returning(move |_| Err(ERR_MSG.into()));
	mock_git_wrapper.expect_add_status_to_editmsg().times(0);
	mock_editmsg_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_stop_and_report_add_status_to_editmsg_error(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper
		.expect_add_status_to_editmsg()
		.returning(move || Err(ERR_MSG.into()));
	mock_editmsg_editor.expect_open().times(0);
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_stop_and_report_when_editor_cannot_be_opened(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(move || Err(ERR_MSG.into()));
	mock_hook_runner.expect_run_commit_msg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_stop_and_report_commit_msg_hook_failure(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_hook_runner
		.expect_run_commit_msg()
		.returning(move || Err(ERR_MSG.into()));
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_report_commit_error(
	mut mock_hook_runner: MockHookRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_git_wrapper.expect_commit().returning(move || Err(ERR_MSG.into()));

	let result = do_commit(&GitService::new(
		mock_git_wrapper,
		mock_hook_runner,
		mock_editmsg_editor,
	));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

fn do_commit<W: GitWrapper, H: HookRunner, E: EditmsgEditor>(service: &GitService<W, H, E>) -> Result<()> {
	service.commit(CommitMode::WithEditor {
		message: Some("a message"),
		authors: vec!["an author".to_string()],
	})
}