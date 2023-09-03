use git::{
	git_domain::{CommitBody, GitWrapper},
	libgit_wrapper::LibGitWrapper,
};
use git2::{Repository, RepositoryInitOptions};
use std::{
	fs::{self, File},
	path::{Path, PathBuf},
	process::Command,
};

#[test]
fn should_determine_if_is_valid_git_repo() {
	let repo_from_sub_dir = LibGitWrapper::new(PathBuf::from(".").canonicalize().unwrap());
	assert!(repo_from_sub_dir.open_if_valid().is_some());

	let repo_from_root_dir = LibGitWrapper::new(PathBuf::from("../..").canonicalize().unwrap());
	assert!(repo_from_root_dir.open_if_valid().is_some());

	let invalid_repo = LibGitWrapper::new(PathBuf::from("/path"));
	assert!(invalid_repo.open_if_valid().is_none());
}

#[test]
fn should_create_a_commit_on_an_already_existing_git_repo_with_staged_changes() {
	let repo_path = "/var/tmp/coa";
	let git_repo = prepare_mock_git_repo(repo_path);
	add_change_to_git_tree(&git_repo);

	let repo = LibGitWrapper::from(git_repo);
	let authors = vec!["random author".to_string()];
	let commit_body = CommitBody::new("irrelevant message", authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", repo_path);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.commit();

	assert!(result.is_ok());
}

#[test]
fn should_error_out_if_no_changes_are_staged() {
	let repo_path = "/var/tmp/coa_err";
	let git_repo = prepare_mock_git_repo(repo_path);

	let repo = LibGitWrapper::from(git_repo);
	let authors = vec!["random author".to_string()];
	let commit_body = CommitBody::new("irrelevant message", authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", repo_path);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.commit();

	assert!(result.unwrap_err().contains("No changes staged for commit"));
}

#[test]
fn should_error_out_if_commit_body_is_empty() {
	let repo_path = "/var/tmp/coa_empty";
	let git_repo = prepare_mock_git_repo(repo_path);
	add_change_to_git_tree(&git_repo);

	let repo = LibGitWrapper::from(git_repo);
	let no_authors = vec!["".to_string()];
	let commit_body = CommitBody::new("", no_authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", repo_path);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.commit();

	assert!(result.unwrap_err().contains("Commit message cannot be empty"));
}

#[test]
fn test_prepares_editmsg_file() {
	let git_repo = prepare_complex_mock_git_repo("/var/tmp/coa_file");
	let repo = LibGitWrapper::from(git_repo);
	repo.add_status_to_editmsg().unwrap();

	let commit_editmsg_path = "/var/tmp/coa_file/.git/COMMIT_EDITMSG";
	let contents = std::fs::read_to_string(&Path::new(commit_editmsg_path));
	assert_eq!(
		contents.unwrap(),
		"

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

fn prepare_mock_git_repo(path: &str) -> Repository {
	let git_repo = init_repo(path);
	add_commit(&git_repo);
	return git_repo;
}

fn init_repo(path: &str) -> Repository {
	let dir = PathBuf::from(path);
	fs::remove_dir_all(&dir).ok();
	return Repository::init_opts(&dir, &RepositoryInitOptions::new()).unwrap();
}

fn add_commit(repo: &Repository) {
	let mut index = repo.index().unwrap();
	let id = index.write_tree().unwrap();
	let tree = repo.find_tree(id).unwrap();
	let sig = repo.signature().unwrap();
	repo.commit(Some("HEAD"), &sig, &sig, "initial commit", &tree, &[])
		.unwrap();
}

fn add_change_to_git_tree(git_repo: &Repository) {
	// Create file
	let root = git_repo.path().parent().unwrap();
	File::create(&root.join("foo")).unwrap();
	// Add to tree
	let mut index = git_repo.index().unwrap();
	index.add_path(Path::new("foo")).unwrap();
	index.write_tree().unwrap();
}

fn prepare_complex_mock_git_repo(path: &str) -> Repository {
	let git_repo = init_repo(path);
	add_change_to_git_tree(&git_repo);
	add_commit(&git_repo);
	let root = git_repo.path().parent().unwrap();

	std::fs::write(&root.join("foo"), "text").unwrap();
	std::fs::write(&root.join("bar"), "text").unwrap();
	std::fs::write(&root.join("baz"), "text").unwrap();

	// TODO. Find a better way to do this, should be able to use git2.rs
	let cwd = std::env::current_dir().unwrap();
	std::env::set_current_dir(path).unwrap();
	Command::new("git")
		.arg("restore")
		.arg("--staged")
		.arg("foo")
		.output()
		.expect("ERR_TEST");
	Command::new("git").arg("add").arg("bar").output().expect("ERR_TEST");
	std::env::set_current_dir(cwd).unwrap();

	return git_repo;
}
