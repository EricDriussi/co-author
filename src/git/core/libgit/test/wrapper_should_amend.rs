use crate::common::fs::file_reader::FileReader;
use crate::error::{assert_error_contains_msg, assert_error_type};
use crate::git::core::commit_message::{CommitMessage, GitWrapper};
use crate::git::core::libgit::test::helper::{
	count_commits, create_and_add_file_to_git_tree, init_repo, random_tmp_path_in, TEST_DIR_PATH,
};
use crate::git::core::libgit::wrapper::LibGitWrapper;
use crate::git::err::GitError;
use crate::Result;
use std::fs::{self};

#[test]
fn on_an_already_existing_git_repo_with_staged_changes() -> Result<()> {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let git_repo = init_repo(&path)?;
	create_and_add_file_to_git_tree(&git_repo, "foo")?;

	let commit_message = CommitMessage::new("irrelevant message", vec!["irrelevant author".to_string()]);
	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", path.to_string_lossy());
	std::fs::write(editmsg_path, commit_message.to_string())?;

	let amount_of_commits_before = count_commits(&path)?;
	let repo = LibGitWrapper::from(&path, FileReader)?;
	let result = repo.amend();
	let amount_of_commits_after = count_commits(&path)?;

	fs::remove_dir_all(path).ok();
	assert!(result.is_ok());
	assert_eq!(amount_of_commits_after, amount_of_commits_before);
	Ok(())
}

#[test]
fn not_if_message_is_empty() -> Result<()> {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let git_repo = init_repo(&path)?;
	create_and_add_file_to_git_tree(&git_repo, "foo")?;

	let empty_commit_message = CommitMessage::new("", vec![String::new()]);
	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", path.to_string_lossy());
	std::fs::write(editmsg_path, empty_commit_message.to_string())?;

	let repo = LibGitWrapper::from(&path, FileReader)?;
	let result = repo.amend();

	fs::remove_dir_all(path).ok();
	assert_error_type(&result, &GitError::LibGit(String::new()));
	assert_error_contains_msg(&result, "Commit message cannot be empty");
	Ok(())
}
