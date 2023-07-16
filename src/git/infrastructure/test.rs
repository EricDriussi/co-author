use super::Git2Repo;
use crate::git::domain::{CommitBody, GitRepo};
use git2::{Repository, RepositoryInitOptions};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[test]
fn should_create_a_commit_on_an_already_existing_git_repo_with_staged_changes() {
    let git_repo = prepare_mock_git_repo("/var/tmp/coa_ok");
    add_file_to_git_tree(&git_repo);

    let repo = Git2Repo::new(git_repo);
    let authors = vec!["random author".to_string()];
    let commit_body = CommitBody::new("irrelevant message", authors);

    let result = repo.commit(commit_body);

    assert!(result.is_ok());
}

#[test]
fn should_error_out_if_no_changes_are_staged() {
    let git_repo = prepare_mock_git_repo("/var/tmp/coa_err");

    let repo = Git2Repo::new(git_repo);
    let authors = vec!["random author".to_string()];
    let commit_body = CommitBody::new("irrelevant message", authors);

    let result = repo.commit(commit_body);

    assert!(result.is_err());
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

fn add_file_to_git_tree(git_repo: &Repository) {
    // Create file
    let root = git_repo.path().parent().unwrap();
    File::create(&root.join("foo")).unwrap();
    // Add to tree
    let mut index = git_repo.index().unwrap();
    index.add_path(Path::new("foo")).unwrap();
    index.write_tree().unwrap();
}
