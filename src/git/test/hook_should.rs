use crate::Result;
use crate::{
	common::runner::MockRunner,
	git::hook::{Hook, HookRunner},
};
use mockall::predicate::{always, eq};
use parameterized::parameterized;

#[parameterized(
	run_fn = { Hook::run_pre_commit, Hook::run_commit_msg },
	hook_name = { "pre-commit", "commit-msg" }
)]
fn call_runner_with_correct_hook(run_fn: fn(&Hook<MockRunner>) -> Result<()>, hook_name: &str) {
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.withf(move |_, hook| hook.contains(hook_name))
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	assert!(result.is_ok());
}

#[parameterized( run_fn = { Hook::run_pre_commit, Hook::run_commit_msg })]
fn call_runner_with_correct_shell(run_fn: fn(&Hook<MockRunner>) -> Result<()>) {
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.with(eq("sh"), always())
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	assert!(result.is_ok());
}
