use std::{env, path::PathBuf};

pub mod app_service;
pub mod author;
pub mod fs_repo;

pub fn default_fs_setup() -> app_service::AuthorsService<fs_repo::FSRepo> {
    let authors_file = config_dir_or_cwd();
    let repo = fs_repo::FSRepo::new(authors_file);
    return app_service::AuthorsService::new(repo);
}

fn config_dir_or_cwd() -> PathBuf {
    let home_dir = env::var("HOME").unwrap();
    let file_path = PathBuf::from(format!("{}/.config/coa/authors", home_dir));
    return if file_path.exists() {
        file_path
    } else {
        PathBuf::from(env::current_dir().unwrap())
    };
}

pub fn custom_fs_setup(authors_file_path: &str) -> app_service::AuthorsService<fs_repo::FSRepo> {
    let repo = fs_repo::FSRepo::new(PathBuf::from(authors_file_path));
    return app_service::AuthorsService::new(repo);
}
