use serial_test::serial;
use std::path::PathBuf;

use git::{
	app_service::GitService,
	git::{CommitBody, GitRepo},
};
use git2::Config;

#[test]
fn should_commit() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let commit_message = "something";
	let aliases = vec![String::from("a")];

	let result = service.commit(commit_message, aliases);

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_commit_using_editor() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let editmsg = ".git/COMMIT_EDITMSG_TEST_OK";
	let editmsg_from_root = format!("../../{}", editmsg);
	std::fs::write(editmsg_from_root.clone(), "himom").unwrap();
	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "not_real").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = service.commit_with_editor(aliases, editmsg);

	assert!(result.is_ok());
	// Cleanup
	std::fs::remove_file(editmsg_from_root).unwrap();
	config.remove("core.editor").unwrap();
}

#[test]
#[serial]
fn should_not_allow_editor_commit_with_no_message() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let editmsg = ".git/COMMIT_EDITMSG_TEST_ERR";
	let editmsg_from_root = format!("../../{}", editmsg);
	std::fs::write(editmsg_from_root.clone(), "").unwrap();
	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "not_real").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = service.commit_with_editor(aliases, editmsg);

	assert!(result.is_err());
	// Cleanup
	std::fs::remove_file(editmsg_from_root).unwrap();
	config.remove("core.editor").unwrap();
}

struct MockRepo {}

impl MockRepo {
	fn new() -> Self {
		Self {}
	}
}

impl GitRepo for MockRepo {
	fn commit(&self, _body: CommitBody) -> Result<(), String> {
		return Ok(());
	}

	fn root(&self) -> Result<PathBuf, String> {
		return Ok(PathBuf::from("../../"));
	}
}
