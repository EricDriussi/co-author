use crate::common::runner::MockRunner;
use crate::error::assert_error_type;
use crate::git::core::hook::{Hook, HookRunner};
use crate::git::err::GitError;
use crate::Result;
use mockall::predicate::{always, eq};
use parameterized::parameterized;
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[parameterized(
	run_fn = { Hook::run_pre_commit, Hook::run_commit_msg },
	hook_name = { "pre-commit", "commit-msg" }
)]
#[serial]
fn call_runner_with_correct_script(run_fn: fn(&Hook<MockRunner>) -> Result<()>, hook_name: &str) {
	let path_with_hooks = create_hooks();
	std::env::set_current_dir(PathBuf::from(path_with_hooks.clone())).expect("Could not set current dir for tests");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.times(1)
		.withf(move |_, hook| hook.contains(hook_name))
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	fs::remove_dir_all(path_with_hooks).expect("Could not remove random path for test");
	assert!(result.is_ok());
}

#[parameterized( run_fn = { Hook::run_pre_commit, Hook::run_commit_msg })]
#[serial]
fn call_runner_with_correct_shell(run_fn: fn(&Hook<MockRunner>) -> Result<()>) {
	let path_with_hooks = create_hooks();
	std::env::set_current_dir(PathBuf::from(path_with_hooks.clone())).expect("Could not set current dir for tests");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.times(1)
		.with(eq("sh"), always())
		.returning(|_, _| Ok(()));

	let result = run_fn(&Hook::new(mock_runner));

	fs::remove_dir_all(path_with_hooks).expect("Could not remove random dir for test");
	assert!(result.is_ok());
}

#[parameterized( run_fn = { Hook::run_pre_commit, Hook::run_commit_msg })]
#[serial]
fn map_error_if_shell_fails(run_fn: fn(&Hook<MockRunner>) -> Result<()>) {
	let path_with_hooks = create_hooks();
	std::env::set_current_dir(PathBuf::from(path_with_hooks.clone())).expect("Could not set current dir for tests");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_run()
		.times(1)
		.with(eq("sh"), always())
		.returning(|_, _| Err("".into()));

	let result = run_fn(&Hook::new(mock_runner));

	fs::remove_dir_all(path_with_hooks).expect("Could not remove random dir for test");
	assert_error_type(&result, &GitError::Hook(String::new()));
}

#[parameterized( run_fn = { Hook::run_pre_commit, Hook::run_commit_msg })]
#[serial]
fn not_run_if_hook_does_not_exist(run_fn: fn(&Hook<MockRunner>) -> Result<()>) {
	let mut mock_runner = MockRunner::new();
	mock_runner.expect_run().times(0);

	let result = run_fn(&Hook::new(mock_runner));

	assert!(result.is_ok());
}

pub fn create_hooks() -> String {
	let dir_path = format!("/tmp/coa/hook/{}", Uuid::new_v4());
	let hook_path = format!("{dir_path}/.git/hooks");
	let pre_commit_path = format!("{hook_path}/pre-commit");
	let commit_msg_path = format!("{hook_path}/commit-msg");

	fs::create_dir_all(hook_path.clone()).expect("Could not create random dir for test");
	fs::File::create(pre_commit_path).expect("Could not create pre-commit hook for test");
	fs::File::create(commit_msg_path).expect("Could not create commit-msg hook for test");
	dir_path
}
