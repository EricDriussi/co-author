use crate::{
	common::fs::{
		file::{File, Locatable, Readable, Writable},
		wrapper::MockFileLoader,
	},
	git::{commit_message::MockGitWrapper, editor::MockEditmsgEditor, hook::MockHookRunner},
};
use mockall::*;

pub fn file_loader_loading(file: MockFile) -> MockFileLoader {
	let mut mock_file_loader = MockFileLoader::new();
	let mut mock_file_opt = Some(file);
	#[allow(clippy::unwrap_used)]
	mock_file_loader
		.expect_load()
		// This is an ugly workaround to appease the borrow checker
		.returning(move |_| Some(Box::new(mock_file_opt.take().unwrap())));
	mock_file_loader
}

pub fn ok_file() -> MockFile {
	let mut mock_file = MockFile::new();
	mock_file.expect_write().returning(|_| Ok(()));
	mock_file
}

pub fn ok_hook_runner() -> MockHookRunner {
	let mut mock_hook_runner = MockHookRunner::new();
	mock_hook_runner.expect_run_pre_commit().returning(|| Ok(()));
	mock_hook_runner.expect_run_commit_msg().returning(|| Ok(()));
	mock_hook_runner
}

pub fn ok_git_wrapper(status: String) -> MockGitWrapper {
	let mut mock_git_wrapper = MockGitWrapper::new();
	mock_git_wrapper
		.expect_formatted_status()
		.returning(move || Ok(status.clone()));
	mock_git_wrapper.expect_commit().returning(|| Ok(()));
	mock_git_wrapper
}

pub fn ok_editor() -> MockEditmsgEditor {
	let mut mock_editmsg_editor = MockEditmsgEditor::new();
	mock_editmsg_editor.expect_open().returning(|| Ok(()));
	mock_editmsg_editor
}

mock! {
	pub File {}

	impl Readable for File {
		fn non_empty_lines(&self) -> Vec<String>;
	}

	impl Writable for File {
		fn write(&mut self, data: String) -> crate::Result<()>;
	}

	impl Locatable for File {
		fn path(&self) -> &str;
	}

	impl File for File {}
}
