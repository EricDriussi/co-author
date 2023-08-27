use super::*;
use git2::Config;
use serial_test::serial;

// TODO.test fallback logic?
#[test]
#[serial]
fn test_get_trimmed_commit_from_editor() {
	let commit_editmsg_path = "../../.git/COMMIT_EDITMSG_TEST";
	let test_commit_message = "  Test commit message.\nThis is a second line. \n".to_string();
	std::fs::write(commit_editmsg_path.clone(), test_commit_message.clone()).unwrap();

	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "not_real").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = get_commit_message_from_editor(PathBuf::from(commit_editmsg_path.clone()));

	assert_eq!(result, Some(test_commit_message.trim().to_string()));

	// Cleanup
	std::fs::remove_file(commit_editmsg_path).unwrap();
	config.remove("core.editor").unwrap();
}

#[test]
#[serial]
fn test_removes_commented_lines_when_getting_commit_from_editor() {
	let commit_editmsg_path = "../../.git/COMMIT_EDITMSG_TEST_COMMENTS";
	let test_commit_message = "Test commit message.\n# This is a commented line.\n#And another one.".to_string();
	std::fs::write(commit_editmsg_path.clone(), test_commit_message.clone()).unwrap();

	let mut config = Config::open_default().unwrap();
	config.set_str("core.editor", "not_real").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = get_commit_message_from_editor(PathBuf::from(commit_editmsg_path.clone()));

	assert_eq!(result, Some("Test commit message.".to_string()));

	// Cleanup
	std::fs::remove_file(commit_editmsg_path).unwrap();
	config.remove("core.editor").unwrap();
}
