use crate::git::{commit_body::MockGitWrapper, editor::MockEditmsgEditor, runner::MockRunner, service::GitService};

#[test]
fn should_commit() {
	let mut mock_git_wrapper = MockGitWrapper::new();
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("pre-commit"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_write_to_editmsg().returning(|_| Ok(()));
	mock_runner
		.expect_run()
		.withf(|_, hook| hook.contains("commit-msg"))
		.returning(|_, _| Ok(()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	let service = GitService::new(mock_git_wrapper, mock_runner, MockEditmsgEditor::new());

	let result = service.commit("a message", vec!["an author".to_string()]);

	assert!(result.is_ok());
}
