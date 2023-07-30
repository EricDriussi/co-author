use std::env;

pub mod app_service;
pub mod author;
pub mod fs_repo;

pub fn fs_setup() -> app_service::AuthorsService<fs_repo::FSRepo> {
    let home_dir = env::var("HOME").unwrap();
    // TODO.handle author file location by param
    let file_path = format!("{}/.config/coa/authors", home_dir);

    let repo = fs_repo::FSRepo::new(file_path.as_str());
    return app_service::AuthorsService::new(repo);
}
