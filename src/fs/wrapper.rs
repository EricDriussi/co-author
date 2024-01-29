use std::env;

use crate::conf;

use super::file::{File, OptionalReadable};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn load_file(&self, file_path: String) -> OptionalReadable;
	fn load_creating(&self, file_path: String) -> OptionalReadable;
	fn load_file_with_fallback(&self, file_path: String) -> OptionalReadable;
}

pub struct FsWrapper {}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}

impl FileLoader for FsWrapper {
	fn load_file(&self, file_path: String) -> OptionalReadable {
		File::from(file_path)
	}

	fn load_creating(&self, file_path: String) -> OptionalReadable {
		File::open_or_create(file_path)
	}

	fn load_file_with_fallback(&self, file_path: String) -> OptionalReadable {
		let authors_dir = conf::authors_dir();
		if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
			return File::from(format!("{xdg_config}/{authors_dir}/{file_path}"));
		}

		let home = env::var("HOME").ok()?;
		File::from(format!("{home}/.config/{authors_dir}/{file_path}"))
			.or_else(|| File::from(format!("{home}/.{authors_dir}/{file_path}")))
			.or_else(|| File::from(format!("{home}/{file_path}")))
	}
}
