use crate::git::{
	hook::{Hook, HookRunner},
	runner::MockRunner,
	test::fixtures::mock_runner,
};
use crate::Result;
use mockall::predicate::{always, eq};
use rstest::*;

#[rstest]
#[case(Hook::run_pre_commit, "pre-commit")]
#[case(Hook::run_commit_msg, "commit-msg")]
fn should_call_runner_with_correct_hook(
	#[case] run_fn: fn(&Hook<MockRunner>) -> Result<()>,
	#[case] hook_name: String,
	mut mock_runner: MockRunner,
) {
	mock_runner
		.expect_run()
		.withf(move |_, hook| hook.contains(&hook_name))
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	assert!(result.is_ok());
}

#[rstest]
#[case(Hook::run_pre_commit)]
#[case(Hook::run_commit_msg)]
fn should_call_runner_with_correct_shell(
	#[case] run_fn: fn(&Hook<MockRunner>) -> Result<()>,
	mut mock_runner: MockRunner,
) {
	mock_runner
		.expect_run()
		.with(eq("sh"), always())
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	assert!(result.is_ok());
}
