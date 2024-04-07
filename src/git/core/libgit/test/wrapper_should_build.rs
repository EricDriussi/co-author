use crate::common::fs::file_reader::FileReader;
use crate::git::core::libgit::test::helper::{
	create_and_add_file_to_git_tree, init_repo, random_tmp_path_in, TEST_DIR_PATH,
};
use crate::git::core::libgit::wrapper::LibGitWrapper;
use std::fs::{self};
use std::path::PathBuf;

#[test]
fn in_repo_with_changes() {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let git_repo = init_repo(&path).expect("Could not create test repo");

	create_and_add_file_to_git_tree(&git_repo, "foo").expect("Could not setup test repo");
	let repo = LibGitWrapper::from(&path, FileReader::new());
	fs::remove_dir_all(path).ok();
	assert!(repo.is_ok());
}

#[test]
fn not_in_repo_without_changes() {
	let path = random_tmp_path_in(TEST_DIR_PATH);
	let _git_repo = init_repo(&path).expect("Could not create test repo");

	let repo = LibGitWrapper::from(&path, FileReader::new());
	fs::remove_dir_all(path).ok();
	assert!(repo.is_err());
}

#[test]
fn not_in_path_wihtout_git_repo() {
	let path = random_tmp_path_in(TEST_DIR_PATH);

	let repo = LibGitWrapper::from(&PathBuf::from("/a/path"), FileReader::new());
	fs::remove_dir_all(path).ok();
	assert!(repo.is_err());
}
