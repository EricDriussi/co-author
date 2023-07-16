use crate::git::domain::{CommitBody, GitRepo};

use git2::{Repository, RepositoryInitOptions};

use std::{
    env,
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
};

use super::Git2Repo;

#[test]
fn commit_should() {
    let (repo_dir, git_repo) = init_repo_with_commit().unwrap();
    let root = git_repo.path().parent().unwrap();
    File::create(&root.join("foo")).unwrap();
    let index = &mut git_repo.index().unwrap();
    index.add_path(Path::new("foo")).unwrap();
    index.write_tree().unwrap();

    let authors = Vec::from(["value".to_string()]);

    let repo = Git2Repo::new(git_repo);

    let commit_body = CommitBody::new("himom", authors);

    let result = repo.commit(commit_body);

    assert!(result.is_ok());

    fs::remove_dir_all(repo_dir).unwrap();
}

fn init_repo_with_commit() -> Result<(PathBuf, Repository), Box<dyn Error>> {
    let home_dir = env::var("HOME").unwrap();
    let tmpdir = format!("{}/.local/share/tmp", home_dir);
    env::set_var("TMPDIR", tmpdir);

    let tmp = env::temp_dir();
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");
    let repo = Repository::init_opts(&tmp, &opts).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "name").unwrap();
        config.set_str("user.email", "email").unwrap();
        let mut index = repo.index().unwrap();
        let id = index.write_tree().unwrap();

        let tree = repo.find_tree(id).unwrap();
        let sig = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
            .unwrap();
    }

    Ok((tmp, repo))
}
