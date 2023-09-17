use serial_test::serial;
use std::error::Error;

use git::{git::GitWrapper, service::GitService};
use git2::Config;

#[test]
fn should_commit() {
	let spy = MockWrapper::new();
	let service = GitService::new(spy);
	let commit_message = "something";
	let aliases = vec![String::from("a")];

	let result = service.commit(commit_message, aliases);

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_commit_using_git_editor() {
	let spy = MockWrapper::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let editmsg = ".git/COMMIT_EDITMSG_TEST";
	let editmsg_from_root = format!("../../{}", editmsg);
	std::fs::write(editmsg_from_root.clone(), "himom").unwrap();
	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "echo").unwrap();

	let result = service.commit_with_editor(aliases);

	assert!(result.is_ok());
	// Cleanup
	std::fs::remove_file(editmsg_from_root).unwrap();
	config.remove("core.editor").unwrap();
}

#[test]
#[serial]
fn should_commit_using_env_editor() {
	let spy = MockWrapper::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let editmsg = ".git/COMMIT_EDITMSG_TEST";
	let editmsg_from_root = format!("../../{}", editmsg);
	std::fs::write(editmsg_from_root.clone(), "himom").unwrap();
	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "NOT_REAL").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = service.commit_with_editor(aliases);

	assert!(result.is_ok());
	// Cleanup
	std::fs::remove_file(editmsg_from_root).unwrap();
	config.remove("core.editor").unwrap();
}

#[test]
fn should_return_last_commit_message_if_present() {
	let last_commit = "msg";
	let wrapper = MockWrapper::with_last_commit(last_commit.to_string());
	let service = GitService::new(wrapper);

	let last_msg = service.last_commit_message();

	assert_eq!(last_msg, last_commit);
}

#[test]
fn should_return_empty_string_if_last_commit_is_not_present() {
	let wrapper = MockWrapper::with_last_commit_err();
	let service = GitService::new(wrapper);

	let last_msg_empty = service.last_commit_message();

	assert_eq!(last_msg_empty, "");
}

struct MockWrapper {
	last_commit: Result<String, Box<dyn Error>>,
}

impl MockWrapper {
	fn new() -> Self {
		Self {
			last_commit: Ok("".to_string()),
		}
	}

	fn with_last_commit(msg: String) -> Self {
		Self { last_commit: Ok(msg) }
	}

	fn with_last_commit_err() -> Self {
		Self {
			last_commit: Err("err".into()),
		}
	}
}

impl GitWrapper for MockWrapper {
	fn commit(&self) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn write_to_editmsg(&self, _: git::git::CommitBody) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn prev_commit_msg(&self) -> Result<String, Box<dyn Error>> {
		return match &self.last_commit {
			Ok(msg) => Ok(msg.into()),
			Err(_) => Err("".into()),
		};
	}
}
