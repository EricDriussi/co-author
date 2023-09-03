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

// FIXME: These tests are flaky and suck, make them better

#[test]
fn should_determine_if_is_valid_git_repo() {
	let repo_path = "/var/tmp/coa_valid";
	prepare_mock_git_repo(repo_path);

	let repo_with_no_staged_changes = LibGitWrapper::from(PathBuf::from(repo_path));
	assert!(repo_with_no_staged_changes.is_err());

	add_change_to_git_tree(repo_path);
	let valid_repo = LibGitWrapper::from(PathBuf::from(repo_path));
	assert!(valid_repo.is_ok());

	let invalid_repo = LibGitWrapper::from(PathBuf::from("/path"));
	assert!(invalid_repo.is_err());
}

#[test]
fn should_create_a_commit_on_an_already_existing_git_repo_with_staged_changes() {
	let repo_path = "/var/tmp/coa";
	prepare_mock_git_repo(repo_path);
	add_change_to_git_tree(repo_path);

	let repo = LibGitWrapper::from(PathBuf::from(repo_path));
	assert!(repo.is_ok());
	let authors = vec!["random author".to_string()];
	let commit_body = CommitBody::new("irrelevant message", authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", repo_path);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.unwrap().commit();

	assert!(result.is_ok());
}

#[test]
fn should_error_out_if_commit_body_is_empty() {
	let repo_path = "/var/tmp/coa_empty";
	prepare_mock_git_repo(repo_path);
	add_change_to_git_tree(repo_path);

	let repo = LibGitWrapper::from(PathBuf::from(repo_path));
	assert!(repo.is_ok());
	let no_authors = vec!["".to_string()];
	let commit_body = CommitBody::new("", no_authors);

	let editmsg_path = format!("{}/.git/COMMIT_EDITMSG", repo_path);
	std::fs::write(editmsg_path, commit_body.formatted_body()).unwrap();

	let result = repo.unwrap().commit();

	assert!(result.unwrap_err().contains("Commit message cannot be empty"));
}

#[test]
fn test_prepares_editmsg_file() {
	let repo_path = "/var/tmp/coa_file";
	prepare_complex_mock_git_repo(repo_path);
	let repo = LibGitWrapper::from(PathBuf::from(repo_path));
	assert!(repo.is_ok());
	repo.unwrap().add_status_to_editmsg().unwrap();

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

fn add_change_to_git_tree(repo_path: &str) {
	File::create(PathBuf::from(repo_path).join("foo")).unwrap();
	std::env::set_current_dir(PathBuf::from(repo_path)).unwrap();
	Command::new("git").arg("add").arg("foo").output().expect("ERR_TEST");
}

fn prepare_complex_mock_git_repo(path: &str) -> Repository {
	let git_repo = init_repo(path);
	add_change_to_git_tree(path);
	add_commit(&git_repo);
	let root = git_repo.path().parent().unwrap();

	std::fs::write(&root.join("foo"), "text").unwrap();
	std::fs::write(&root.join("bar"), "text").unwrap();
	std::fs::write(&root.join("baz"), "text").unwrap();

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
