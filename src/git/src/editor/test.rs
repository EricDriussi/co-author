use super::*;

#[test]
fn test_get_commit_from_editor() {
	// TODO. Refactor, depends on FIXME in src/git/src/editor/mod.rs:12
	let commit_editmsg = ".git/COMMIT_EDITMSG_TEST";
	let commit_editmsg_path = format!("../../{}", commit_editmsg);
	let test_commit_message = "Test commit message.\nThis is a second line.\n".to_string();
	std::fs::write(commit_editmsg_path.clone(), test_commit_message.clone()).unwrap();

	std::env::set_var("EDITOR", "echo");

	let result = get_commit_from_editor(String::from(commit_editmsg));

	assert_eq!(result, Some(test_commit_message));

	// Cleanup
	std::fs::remove_file(commit_editmsg_path).unwrap();
}
