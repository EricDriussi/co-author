use std::env;

use mockall::predicate::{always, eq};
use serial_test::serial;

use crate::{
	common::{conf, fs::wrapper::MockFileLoader, runner::MockRunner},
	git::{
		conf_provider::MockConfProvider,
		editor::{EditmsgEditor, Editor},
	},
	test_utils::dummy_file::DummyFile,
};

#[test]
fn should_get_editmsg_from_conf() {
	let mut mock_runner = MockRunner::new();
	mock_runner.expect_spawn().returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some("an_editor".to_string()));
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.with(eq(conf::editmsg().clone()))
		.returning(move |_| Some(Box::new(DummyFile::empty())));
	let editor = Editor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open();

	assert!(result.is_ok());
}

#[test]
fn should_error_when_no_editmsg_is_found() {
	let mut mock_runner = MockRunner::new();
	mock_runner.expect_spawn().returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some("an_editor".to_string()));
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.with(eq(conf::editmsg().clone()))
		.returning(move |_| None);
	let editor = Editor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open();

	assert!(matches!(result, Err(e) if e.to_string().contains("Editor")));
}

#[test]
fn should_open_with_git_conf_editor() {
	let a_user_editor = "an_editor";
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some(a_user_editor.to_string()));
	let editor = Editor::new(
		successful_runner_for(a_user_editor),
		successful_mock_file_loader(),
		mock_conf_provider,
	);

	let result = editor.open();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_env_editor() {
	let a_user_editor = "an_editor";
	env::set_var("EDITOR", a_user_editor);
	let editor = Editor::new(
		successful_runner_for(a_user_editor),
		successful_mock_file_loader(),
		mock_conf_provider_with_no_editor(),
	);

	let result = editor.open();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_vim_editor() {
	env::remove_var("EDITOR");
	let editor = Editor::new(
		successful_runner_for("vim"),
		successful_mock_file_loader(),
		mock_conf_provider_with_no_editor(),
	);

	let result = editor.open();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_vi_editor() {
	env::remove_var("EDITOR");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_spawn()
		.with(eq("vim"), always())
		.returning(|_, _| Err("ERROR".into()));
	mock_runner
		.expect_spawn()
		.with(eq("vi"), always())
		.returning(|_, _| Ok(()));
	let editor = Editor::new(
		mock_runner,
		successful_mock_file_loader(),
		mock_conf_provider_with_no_editor(),
	);

	let result = editor.open();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_error_when_no_editor_is_available() {
	env::remove_var("EDITOR");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_spawn()
		.with(eq("vim"), always())
		.returning(|_, _| Err("ERROR".into()));
	mock_runner
		.expect_spawn()
		.with(eq("vi"), always())
		.returning(|_, _| Err("ERROR".into()));
	let editor = Editor::new(
		mock_runner,
		successful_mock_file_loader(),
		mock_conf_provider_with_no_editor(),
	);

	let result = editor.open();

	assert!(matches!(result, Err(e) if e.to_string().contains("Editor")));
}

fn successful_mock_file_loader() -> MockFileLoader {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyFile::empty())));
	mock_file_loader
}

fn successful_runner_for(editor: &str) -> MockRunner {
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_spawn()
		.with(eq(editor.to_string()), always())
		.returning(|_, _| Ok(()));
	mock_runner
}

fn mock_conf_provider_with_no_editor() -> MockConfProvider {
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	mock_conf_provider
}
