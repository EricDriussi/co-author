use super::*;

#[test]
fn test_get_commit_from_editor() {
	let commit_editmsg_path = "../../.git/COMMIT_EDITMSG";
	let test_commit_message = "Test commit message.\nThis is a second line.\n".to_string();
	std::fs::write(commit_editmsg_path, test_commit_message.clone()).unwrap();

	std::env::set_var("EDITOR", "echo");

	let result = get_commit_from_editor();

	assert_eq!(result, Some(test_commit_message));

	// Cleanup
	std::fs::remove_file(commit_editmsg_path).unwrap();
}
