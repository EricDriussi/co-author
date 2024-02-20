use std::env;

use crate::common::conf;

use super::file::{OptionalFile, SimpleFile};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn load_file(&self, file_path: String) -> OptionalFile;
	fn load_creating(&self, file_path: String) -> OptionalFile;
	fn load_file_with_fallback(&self, file_path: String) -> OptionalFile;
}

pub struct FsWrapper {}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}

impl FileLoader for FsWrapper {
	fn load_file(&self, file_path: String) -> OptionalFile {
		SimpleFile::from(file_path)
	}

	fn load_creating(&self, file_path: String) -> OptionalFile {
		SimpleFile::open_or_create(file_path)
	}

	fn load_file_with_fallback(&self, file_path: String) -> OptionalFile {
		let authors_dir = conf::authors_dir();
		if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
			return SimpleFile::from(format!("{xdg_config}/{authors_dir}/{file_path}"));
		}

		let home = env::var("HOME").ok()?;
		SimpleFile::from(format!("{home}/.config/{authors_dir}/{file_path}"))
			.or_else(|| SimpleFile::from(format!("{home}/.{authors_dir}/{file_path}")))
			.or_else(|| SimpleFile::from(format!("{home}/{file_path}")))
	}
}
