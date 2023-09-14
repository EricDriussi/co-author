use std::{fs::OpenOptions, io::Write};

use serial_test::serial;
use std::os::unix::fs::OpenOptionsExt;

use super::*;

#[test]
#[serial]
fn should_run_pre_commit_hook() {
	let hooks_dir = "../../.git/hooks";
	let pre_commit_hook_path = format!("{}/pre-commit", hooks_dir);
	let ok_hook_code = "#!/bin/sh\nexit 0";

	create_test_hook(pre_commit_hook_path.as_str(), ok_hook_code);

	let result = pre_commit(PathBuf::from("../../.git/hooks"));

	assert!(result.is_ok());
	// Cleanup
	std::fs::remove_file(pre_commit_hook_path).unwrap();
}

#[test]
#[serial]
fn should_err_pre_commit_hook() {
	let hooks_dir = "../../.git/hooks";
	let pre_commit_hook_path = format!("{}/pre-commit", hooks_dir);
	let err_hook_code = "#!/bin/sh\nexit 1";

	create_test_hook(pre_commit_hook_path.as_str(), err_hook_code);

	let result = pre_commit(PathBuf::from("../../.git/hooks"));

	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "Pre-commit hook failed, aborting");
	// Cleanup
	std::fs::remove_file(pre_commit_hook_path).unwrap();
}

#[test]
#[serial]
fn should_run_commit_msg_hook() {
	let hooks_dir = "../../.git/hooks";
	let commit_msg_hook_path = format!("{}/commit-msg", hooks_dir);
	let ok_hook_code = "#!/bin/sh\necho $1\nexit 0";
	let path_to_editmsg = "NOT_RELEVANT";

	create_test_hook(commit_msg_hook_path.as_str(), ok_hook_code);

	let result = commit_msg(PathBuf::from("../../.git/hooks"), PathBuf::from(path_to_editmsg));

	assert!(result.is_ok());
	// Cleanup
	std::fs::remove_file(commit_msg_hook_path).unwrap();
}

#[test]
#[serial]
fn should_err_commit_msg_hook() {
	let hooks_dir = "../../.git/hooks";
	let commit_msg_hook_path = format!("{}/commit-msg", hooks_dir);
	let err_hook_code = "#!/bin/sh\necho $1\nexit 1";
	let path_to_editmsg = "NOT_RELEVANT";

	create_test_hook(commit_msg_hook_path.as_str(), err_hook_code);

	let result = commit_msg(PathBuf::from("../../.git/hooks"), PathBuf::from(path_to_editmsg));

	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "Commit-msg hook failed, aborting");
	// Cleanup
	std::fs::remove_file(commit_msg_hook_path).unwrap();
}

fn create_test_hook(path: &str, hook_code: &str) {
	let _ = std::fs::remove_file(path);
	let mut file = OpenOptions::new()
		.write(true)
		.create_new(true)
		.mode(0o755)
		.open(path)
		.unwrap();
	file.write_all(hook_code.as_bytes()).unwrap();
	drop(file);
}
