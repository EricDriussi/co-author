use crate::common::fs::file_reader::FileReader;
use crate::git::core::commit_message::GitWrapper;
use crate::git::core::libgit::test::helper::{
	create_and_add_file_to_git_tree, init_repo, random_tmp_path_in, TEST_DIR_PATH,
};
use crate::git::core::libgit::wrapper::LibGitWrapper;
use crate::Result;
use git2::Repository;
use std::fs::{self};

#[test]
fn add_status_to_editmsg_file() -> Result<()> {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let git_repo = init_repo(&path)?;
	let foo = "foo";
	create_and_add_file_to_git_tree(&git_repo, foo)?;

	let tree = git_repo.find_tree(git_repo.index()?.write_tree()?)?;
	// add a commit to track foo
	add_commit(&git_repo, &tree.clone(), "IRRELEVANT")?;

	// create and add bar
	create_and_add_file_to_git_tree(&git_repo, "bar")?;
	// modify foo but don't add changes
	std::fs::write(path.join(foo), "text")?;
	// create baz but keep untracked
	std::fs::write(path.join("baz"), "text")?;

	add_commit(&git_repo, &tree, "IRRELEVANT")?;

	let repo = LibGitWrapper::from(&path, FileReader)?;
	let contents = repo.formatted_status()?;

	fs::remove_dir_all(path).ok();
	assert_eq!(
		contents,
		"

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# A message with only 'Co-Authored' lines will be considered empty.
#
# On branch master
# Changes to be committed:
#	bar
#
# Changes not staged for commit:
#	foo
#
# Untracked files:
#	baz
"
	);
	Ok(())
}

#[test]
fn get_the_last_commit() -> Result<()> {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let git_repo = init_repo(&path)?;
	create_and_add_file_to_git_tree(&git_repo, "foo")?;
	let repo = LibGitWrapper::from(&path, FileReader)?;

	let tree = git_repo.find_tree(git_repo.index()?.write_tree()?)?;
	let msg = "a commit message!".to_string();
	add_commit(&git_repo, &tree, msg.as_str())?;

	let result = repo.prev_commit_msg();

	fs::remove_dir_all(path.clone()).ok();
	assert!(matches!(result, Ok(line) if line.to_string().contains(msg.as_str())));
	Ok(())
}

fn add_commit(repo: &Repository, tree: &git2::Tree<'_>, msg: &str) -> Result<()> {
	let head_commit = repo.head()?.peel_to_commit()?;
	repo.commit(
		Some("HEAD"),
		&repo.signature()?,
		&repo.signature()?,
		msg,
		tree,
		&[&head_commit],
	)?;
	Ok(())
}
