mod config;
pub use config::authors_file;
pub use config::editmsg;
pub use config::get_config;
pub use config::hooks_path;

#[cfg(test)]
mod test;
