use crate::Result;
use git2::{Config, Repository, RepositoryInitOptions, Signature};
use std::fs::File;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub const TEST_DIR_PATH: &str = "/tmp/coa/libgit_wrapper";

pub fn init_repo(path: &PathBuf) -> Result<Repository> {
	let repo = Repository::init_opts(path, &RepositoryInitOptions::new())?;
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

pub fn set_user_and_email(conf: &mut Config) -> Result<()> {
	let sig = Signature::now("a_name", "an_email")?;
	conf.set_str("user.name", sig.name().ok_or("Could not setup signature for test")?)?;
	conf.set_str("user.email", sig.email().ok_or("Could not setup signature for test")?)?;
	Ok(())
}

pub fn create_and_add_file_to_git_tree(repo: &Repository, file_name: &str) -> Result<()> {
	let root = repo.path().parent().ok_or("Could not add file to test git tree")?;
	File::create(root.join(file_name))?;

	let mut index = repo.index()?;
	index.add_path(Path::new(file_name))?;
	index.write()?;
	Ok(())
}

pub fn random_tmp_path_in(path: &str) -> PathBuf {
	let random = Uuid::new_v4();
	PathBuf::from(format!("{path}/{random}"))
}
