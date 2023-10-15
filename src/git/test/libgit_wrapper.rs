use git2::{Repository, RepositoryInitOptions};
use serial_test::serial;
use std::{
	fs::{self, File},
	path::{Path, PathBuf},
};

use crate::git::{
	commit_body::{CommitBody, GitWrapper},
	libgit_wrapper::LibGitWrapper,
};

const REPO_PATH: &str = "/var/tmp/coa";

#[test]
#[serial]
fn should_determine_if_is_valid_git_repo() {
	let git_repo = init_repo(REPO_PATH);

	let repo_with_no_staged_changes = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(repo_with_no_staged_changes.is_err());

	create_and_add_file_to_git_tree(&git_repo, "foo");
	let valid_repo = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(valid_repo.is_ok());

	let invalid_repo = LibGitWrapper::from(PathBuf::from("/path"));
	assert!(invalid_repo.is_err());
}

#[test]
#[serial]
fn should_create_a_commit_on_an_already_existing_git_repo_with_staged_changes() {
	let git_repo = init_repo(REPO_PATH);
	create_and_add_file_to_git_tree(&git_repo, "foo");

	let repo = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(repo.is_ok());
	let authors = vec!["random author".to_string()];
	let commit_body = CommitBody::new("irrelevant message", authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", REPO_PATH);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.unwrap().commit();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_error_out_if_commit_body_is_empty() {
	let git_repo = init_repo(REPO_PATH);
	create_and_add_file_to_git_tree(&git_repo, "foo");

	let repo = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(repo.is_ok());
	let no_authors = vec!["".to_string()];
	let commit_body = CommitBody::new("", no_authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", REPO_PATH);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.unwrap().commit();

	assert!(result
		.unwrap_err()
		.to_string()
		.contains("Commit message cannot be empty"));
}

#[test]
#[serial]
fn test_prepares_editmsg_file() {
	let git_repo = init_repo(REPO_PATH);
	create_and_add_file_to_git_tree(&git_repo, "foo");

	let mut index = git_repo.index().unwrap();
	let id = index.write_tree().unwrap();
	let tree = git_repo.find_tree(id).unwrap();
	add_commit(&git_repo, tree.clone(), "IRRELEVANT");

	// add bar
	create_and_add_file_to_git_tree(&git_repo, "bar");
	// modify but don't add foo
	let root = git_repo.path().parent().unwrap();
	std::fs::write(root.join("foo"), "text").unwrap();
	// add baz but keep untracked
	std::fs::write(root.join("baz"), "text").unwrap();

	add_commit(&git_repo, tree, "IRRELEVANT");

	let repo = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(repo.is_ok());
	repo.unwrap().add_status_to_editmsg().unwrap();

	let commit_editmsg_path = "/var/tmp/coa/.git/COMMIT_EDITMSG";
	let contents = std::fs::read_to_string(Path::new(commit_editmsg_path));
	assert_eq!(
		contents.unwrap(),
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
}

#[test]
#[serial]
fn should_only_return_the_first_line_from_the_last_commit() {
	let git_repo = init_repo(REPO_PATH);
	create_and_add_file_to_git_tree(&git_repo, "foo");

	let mut index = git_repo.index().unwrap();
	let id = index.write_tree().unwrap();
	let tree = git_repo.find_tree(id).unwrap();
	let repo = LibGitWrapper::from(PathBuf::from(REPO_PATH));
	assert!(repo.is_ok());

	let first_line = "FIRST LINE".to_string();
	let msg = format!("{}\nSECOND_LINE", first_line);
	add_commit(&git_repo, tree, msg.as_str());

	let result = repo.unwrap().prev_commit_msg();

	assert_eq!(result.unwrap(), first_line);
}

fn init_repo(path: &str) -> Repository {
	let dir = PathBuf::from(path);
	fs::remove_dir_all(&dir).ok();
	let repo = Repository::init_opts(&dir, &RepositoryInitOptions::new()).unwrap();

	let mut index = repo.index().unwrap();
	let id = index.write_tree().unwrap();
	let tree = repo.find_tree(id).unwrap();
	let sig = repo.signature().unwrap();
	repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
		.unwrap();
	drop(tree);
	repo
}

fn add_commit(repo: &Repository, tree: git2::Tree<'_>, msg: &str) {
	let sig = repo.signature().unwrap();
	let head_commit = repo.head().unwrap().peel_to_commit().unwrap();
	repo.commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&head_commit])
		.unwrap();
}

fn create_and_add_file_to_git_tree(repo: &Repository, file_name: &str) {
	let root = repo.path().parent().unwrap();
	File::create(root.join(file_name)).unwrap();

	let mut index = repo.index().unwrap();
	index.add_path(Path::new(file_name)).unwrap();
	index.write().unwrap();
}
