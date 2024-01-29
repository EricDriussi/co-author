use std::env;

use mockall::predicate::{always, eq};
use serial_test::serial;

use crate::{
	conf,
	fs::{file::Readable, wrapper::MockFileLoader},
	git::{conf_provider::MockConfProvider, editor::TextEditor, git_err::GitError, runner::MockRunner},
};

#[test]
fn should_get_editmsg_from_conf() {
	let mut mock_runner = MockRunner::new();
	mock_runner.expect_open_editor().returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some("an_editor".to_string()));
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.with(eq(conf::editmsg().clone()))
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(result.is_ok());
}

#[test]
fn should_error_when_no_editmsg_is_found() {
	let mut mock_runner = MockRunner::new();
	mock_runner.expect_open_editor().returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some("an_editor".to_string()));
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.with(eq(conf::editmsg().clone()))
		.returning(move |_| None);
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(matches!(result, Err(e) if e.to_string().contains("Editor")));
}

#[test]
fn should_open_with_git_conf_editor() {
	let a_user_editor = "an_editor";
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_open_editor()
		.with(eq(a_user_editor), always())
		.returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider
		.expect_get_editor()
		.returning(|| Some(a_user_editor.to_string()));
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_env_editor() {
	let a_user_editor = "an_editor";
	env::set_var("EDITOR", a_user_editor);
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_open_editor()
		.with(eq(a_user_editor), always())
		.returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_vim_editor() {
	env::remove_var("EDITOR");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_open_editor()
		.with(eq("vim"), always())
		.returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_open_with_vi_editor() {
	env::remove_var("EDITOR");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_open_editor()
		.with(eq("vim"), always())
		.returning(|_, _| Err(Box::new(GitError::Editor)));
	mock_runner
		.expect_open_editor()
		.with(eq("vi"), always())
		.returning(|_, _| Ok(()));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_error_when_no_editor_is_available() {
	env::remove_var("EDITOR");
	let mut mock_runner = MockRunner::new();
	mock_runner
		.expect_open_editor()
		.with(eq("vim"), always())
		.returning(|_, _| Err(Box::new(GitError::Editor)));
	mock_runner
		.expect_open_editor()
		.with(eq("vi"), always())
		.returning(|_, _| Err(Box::new(GitError::Editor)));
	let mut mock_conf_provider = MockConfProvider::new();
	mock_conf_provider.expect_get_editor().returning(|| None);
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load_creating()
		.returning(move |_| Some(Box::new(DummyReadableFile::empty())));
	let editor = TextEditor::new(mock_runner, mock_file_loader, mock_conf_provider);

	let result = editor.open_editmsg();

	assert!(matches!(result, Err(e) if e.to_string().contains("Editor")));
}

// TODO: dup code, extract
pub struct DummyReadableFile {
	content: Vec<String>,
	path: String,
}

impl DummyReadableFile {
	pub fn empty() -> Self {
		Self {
			content: (vec![]),
			path: String::new(),
		}
	}
}

impl Readable for DummyReadableFile {
	fn non_empty_lines(&self) -> Vec<String> {
		self.content.clone()
	}

	fn all_lines(&self) -> Vec<String> {
		self.content.clone()
	}

	fn path(&self) -> &str {
		self.path.as_str()
	}
}
