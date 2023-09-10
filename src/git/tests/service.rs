use std::{error::Error, path::PathBuf};

use git::{git_domain::GitWrapper, service::GitService};
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
fn should_commit_using_editor() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let editmsg = ".git/COMMIT_EDITMSG_TEST";
	let editmsg_from_root = format!("../../{}", editmsg);
	std::fs::write(editmsg_from_root.clone(), "himom").unwrap();
	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "not_real").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = service.commit_with_editor(aliases);

	assert!(result.is_ok());
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

impl GitWrapper for MockRepo {
	fn commit(&self) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn write_to_editmsg(&self, _: git::git_domain::CommitBody) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>> {
		return Ok(());
	}

	fn editmsg_path(&self) -> PathBuf {
		return PathBuf::from("../../.git/COMMIT_EDITMSG_TEST");
	}

	fn hooks_path(&self) -> PathBuf {
		return PathBuf::from("../../.git/COMMIT_EDITMSG_TEST");
	}
}
