mod config;
pub use config::authors_file_name;
pub use config::authors_file_path;
pub use config::dummy_data;
pub use config::editmsg;
pub use config::hooks_path;

#[cfg(test)]
mod test;
