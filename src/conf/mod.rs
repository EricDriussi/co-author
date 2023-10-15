pub mod conf;
pub use conf::authors_file;
pub use conf::editmsg;
pub use conf::get_config;
pub use conf::hooks_path;

#[cfg(test)]
mod test;
