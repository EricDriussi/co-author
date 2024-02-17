use crate::git::{
	commit_body::{CommitBody, GitWrapper, MockGitWrapper},
	editor::{EditmsgEditor, MockEditmsgEditor},
	runner::{MockRunner, Runner},
	service::{CommitMode, GitService},
	test::service::fixtures::{mock_editmsg_editor, mock_git_wrapper, mock_runner, COMMIT_MSG_HOOK, PRE_COMMIT_HOOK},
};
use crate::Result;
use mockall::{predicate::eq, Sequence};
use rstest::*;

const ERR_MSG: &str = "ERR";

#[rstest]
fn should_succeed(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(result.is_ok());
}

#[rstest]
fn should_write_commit_msg_and_authors(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	let message = "a message";
	let authors = vec!["an author".to_string()];
	mock_git_wrapper
		.expect_write_to_editmsg()
		.with(eq(CommitBody::new(message, authors.clone())))
		.returning(|_| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor);

	let result = service.commit(CommitMode::WithoutEditor { message, authors });

	assert!(result.is_ok());
}

#[rstest]
fn should_not_add_status_to_editmsg(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	mock_git_wrapper.expect_add_status_to_editmsg().times(0);

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(result.is_ok());
}

#[rstest]
fn should_not_open_editor(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mut mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	mock_editmsg_editor.expect_open().times(0);

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(result.is_ok());
}

#[rstest]
fn should_perform_actions_in_order(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
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

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(result.is_ok());
}

#[rstest]
fn should_stop_and_report_hook_error(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner
		.expect_run()
		.returning(|_, _| Err("an error".to_string().into()));
	mock_git_wrapper.expect_write_to_editmsg().times(0);
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(matches!(result, Err(e) if e.to_string().contains("Hook")));
}

#[rstest]
fn should_stop_and_report_write_to_editmsg_error(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper
		.expect_write_to_editmsg()
		.returning(move |_| Err(ERR_MSG.into()));
	mock_git_wrapper.expect_commit().times(0);

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

#[rstest]
fn should_report_commit_error(
	mut mock_runner: MockRunner,
	mut mock_git_wrapper: MockGitWrapper,
	mock_editmsg_editor: MockEditmsgEditor,
) {
	mock_runner.expect_run().returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_git_wrapper.expect_commit().returning(move || Err(ERR_MSG.into()));

	let result = do_commit(&GitService::new(mock_git_wrapper, mock_runner, mock_editmsg_editor));

	assert!(matches!(result, Err(e) if e.to_string().contains(ERR_MSG)));
}

fn do_commit<W: GitWrapper, R: Runner, E: EditmsgEditor>(service: &GitService<W, R, E>) -> Result<()> {
	service.commit(CommitMode::WithoutEditor {
		message: "a message",
		authors: vec!["an author".to_string()],
	})
}
