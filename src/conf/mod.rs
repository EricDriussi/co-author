mod config;
pub use config::authors_dir;
pub use config::authors_file;
pub use config::dummy_data;
pub use config::editmsg;
pub use config::hooks_path;

#[cfg(test)]
mod test;
