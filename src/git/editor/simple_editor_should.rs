use crate::{
	common::runner::MockRunner,
	error::assert_error_type,
	git::{
		editor::{
			conf_provider::MockDefaultEditorGetter,
			simple_editor::{Editor, SimpleEditor},
		},
		err::GitError,
	},
};
use mockall::{
	predicate::{always, eq},
	Sequence,
};
use serial_test::serial;
use std::env;

#[test]
fn open_with_git_conf_editor() {
	let a_user_editor = "an_editor";
	let mut mock_conf_provider = MockDefaultEditorGetter::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some(a_user_editor.to_string()));
	let editor = SimpleEditor::new(successful_runner_for(a_user_editor), mock_conf_provider);

	let result = editor.open("");

	assert!(result.is_ok());
}

#[test]
#[serial]
fn open_with_env_editor() {
	let a_user_editor = "an_editor";
	env::set_var("EDITOR", a_user_editor);
	let editor = SimpleEditor::new(
		successful_runner_for(a_user_editor),
		mock_conf_provider_with_no_editor(),
	);

	let result = editor.open("");

	assert!(result.is_ok());
}

#[test]
#[serial]
fn fallback_sensibly() {
	env::remove_var("EDITOR");
	let mut seq = Sequence::new();
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_spawn()
		.with(eq("vim".to_string()), always())
		.returning(|_, _| Err("ERROR".into()))
		.times(1)
		.in_sequence(&mut seq);
	mock_runner
		.expect_spawn()
		.with(eq("vi".to_string()), always())
		.returning(|_, _| Err("ERROR".into()))
		.times(1)
		.in_sequence(&mut seq);
	let editor = SimpleEditor::new(mock_runner, mock_conf_provider_with_no_editor());

	let result = editor.open("");

	assert_error_type(&result, &GitError::Editor);
}

fn successful_runner_for(editor: &str) -> MockRunner {
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_spawn()
		.with(eq(editor.to_string()), always())
		.returning(|_, _| Ok(()));
	mock_runner
}

fn mock_conf_provider_with_no_editor() -> MockDefaultEditorGetter {
	let mut mock_conf_provider = MockDefaultEditorGetter::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	mock_conf_provider
}
