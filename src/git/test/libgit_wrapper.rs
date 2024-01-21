use co_author::test_utils::set_test_env;
use git2::{Config, Repository, RepositoryInitOptions, Signature};
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
	let git_repo = init_repo(REPO_PATH).expect("Could not setup test repo");

	let repo_with_no_staged_changes = LibGitWrapper::from(&PathBuf::from(REPO_PATH));
	assert!(repo_with_no_staged_changes.is_err());

	create_and_add_file_to_git_tree(&git_repo, "foo").expect("Could not setup test repo");
	let valid_repo = LibGitWrapper::from(&PathBuf::from(REPO_PATH));
	assert!(valid_repo.is_ok());

	let invalid_repo = LibGitWrapper::from(&PathBuf::from("/path"));
	assert!(invalid_repo.is_err());
}

#[test]
#[serial]
fn should_create_a_commit_on_an_already_existing_git_repo_with_staged_changes() {
	set_test_env();
	let git_repo = init_repo(REPO_PATH).expect("Could not setup test repo");
	create_and_add_file_to_git_tree(&git_repo, "foo").expect("Could not setup test repo");

	let repo = LibGitWrapper::from(&PathBuf::from(REPO_PATH)).expect("Could not setup test repo");
	let authors = vec!["random author".to_string()];
	let commit_body = CommitBody::new("irrelevant message", authors);

	let editmsg_path = format!("{REPO_PATH}/.git/COMMIT_EDITMSG");
	std::fs::write(editmsg_path, commit_body.formatted_body()).expect("Could not write to test editmsg file");

	let result = repo.commit();

	assert!(result.is_ok());
}

#[test]
#[serial]
fn should_error_out_if_commit_body_is_empty() {
	let git_repo = init_repo(REPO_PATH).expect("Could not setup test repo");
	create_and_add_file_to_git_tree(&git_repo, "foo").expect("Could not setup test repo");

	let repo = LibGitWrapper::from(&PathBuf::from(REPO_PATH)).expect("Could not setup test repo");
	let no_authors = vec![String::new()];
	let commit_body = CommitBody::new("", no_authors);

	let editmsg_path = format!("{REPO_PATH}/.git/COMMIT_EDITMSG");
	std::fs::write(editmsg_path, commit_body.formatted_body()).expect("Could not write to test editmsg file");

	let result = repo.commit();

	assert!(matches!(result, Err(e) if e.to_string().contains("Commit message cannot be empty")));
}

#[test]
#[serial]
fn test_prepares_editmsg_file() -> Result<(), Box<dyn std::error::Error>> {
	set_test_env();
	let git_repo = init_repo(REPO_PATH)?;
	create_and_add_file_to_git_tree(&git_repo, "foo")?;

	let mut index = git_repo.index()?;
	let id = index.write_tree()?;
	let tree = git_repo.find_tree(id)?;
	add_commit(&git_repo, &tree.clone(), "IRRELEVANT")?;

	// add bar
	create_and_add_file_to_git_tree(&git_repo, "bar")?;
	// modify but don't add foo
	let root = git_repo.path().parent().ok_or("Coult not setup editmsg test")?;
	std::fs::write(root.join("foo"), "text")?;
	// add baz but keep untracked
	std::fs::write(root.join("baz"), "text")?;

	add_commit(&git_repo, &tree, "IRRELEVANT")?;

	let repo = LibGitWrapper::from(&PathBuf::from(REPO_PATH))?;
	repo.add_status_to_editmsg()?;

	let commit_editmsg_path = "/var/tmp/coa/.git/COMMIT_EDITMSG";
	let contents = std::fs::read_to_string(Path::new(commit_editmsg_path));
	assert_eq!(
		contents?,
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
#[serial]
fn should_only_return_the_first_line_from_the_last_commit() -> Result<(), Box<dyn std::error::Error>> {
	let git_repo = init_repo(REPO_PATH)?;
	create_and_add_file_to_git_tree(&git_repo, "foo")?;

	let mut index = git_repo.index()?;
	let id = index.write_tree()?;
	let tree = git_repo.find_tree(id)?;
	let repo = LibGitWrapper::from(&PathBuf::from(REPO_PATH))?;

	let first_line = "FIRST LINE".to_string();
	let msg = format!("{first_line}\nSECOND_LINE");
	add_commit(&git_repo, &tree, msg.as_str())?;

	let result = repo.prev_commit_msg();

	assert!(matches!(result, Ok(line) if line.to_string().contains(first_line.as_str())));
	Ok(())
}

fn init_repo(path: &str) -> Result<Repository, Box<dyn std::error::Error>> {
	let dir = PathBuf::from(path);
	fs::remove_dir_all(&dir).ok();
	let repo = Repository::init_opts(&dir, &RepositoryInitOptions::new())?;
	set_user_and_email(&mut repo.config()?)?;

	let mut index = repo.index()?;
	let id = index.write_tree()?;
	let tree = repo.find_tree(id)?;
	repo.commit(
		Some("HEAD"),
		&repo.signature()?,
		&repo.signature()?,
		"initial commit",
		&tree,
		&[],
	)?;
	drop(tree);
	Ok(repo)
}

fn set_user_and_email(conf: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
	let sig = Signature::now("a_name", "an_email")?;
	conf.set_str("user.name", sig.name().ok_or("Could not setup signature for test")?)?;
	conf.set_str("user.email", sig.email().ok_or("Could not setup signature for test")?)?;
	Ok(())
}

fn add_commit(repo: &Repository, tree: &git2::Tree<'_>, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_and_add_file_to_git_tree(repo: &Repository, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
	let root = repo.path().parent().ok_or("Could not add file to test git tree")?;
	File::create(root.join(file_name))?;

	let mut index = repo.index()?;
	index.add_path(Path::new(file_name))?;
	index.write()?;
	Ok(())
}
