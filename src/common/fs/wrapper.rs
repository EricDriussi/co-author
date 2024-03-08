use super::file::{File, SimpleFile};
use crate::common::conf;
use std::{env, fs::OpenOptions};

pub type OptionalFile = Option<Box<dyn File>>;

#[cfg_attr(test, mockall::automock)]
pub trait FileLoader {
	fn load_if_present(&self, file_path: String) -> OptionalFile;
	fn load(&self, file_path: String) -> OptionalFile;
	fn load_with_fallback(&self, file_path: String) -> OptionalFile;
}

pub struct FsWrapper {}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}

	fn home_fallback(&self, authors_dir: &str, file_path: &str) -> OptionalFile {
		let home = env::var("HOME").ok()?;
		self.load_if_present(format!("{home}/.config/{authors_dir}/{file_path}"))
			.or_else(|| self.load_if_present(format!("{home}/.{authors_dir}/{file_path}")))
			.or_else(|| self.load_if_present(format!("{home}/{file_path}")))
	}
}

impl FileLoader for FsWrapper {
	fn load_if_present(&self, file_path: String) -> OptionalFile {
		let file = OpenOptions::new()
			.read(true)
			.append(true)
			.open(file_path.clone())
			.ok()?;
		Some(SimpleFile::from(file, file_path))
	}

	fn load(&self, file_path: String) -> OptionalFile {
		let file = OpenOptions::new()
			.read(true)
			.append(true)
			.create(true)
			.open(file_path.clone())
			.ok()?;
		Some(SimpleFile::from(file, file_path))
	}

	// TODO: this should be in the provider, not here
	fn load_with_fallback(&self, file_path: String) -> OptionalFile {
		let authors_dir = conf::authors_dir();
		match env::var("XDG_CONFIG_HOME") {
			Err(_) => self.home_fallback(&authors_dir, &file_path),
			Ok(xdg_config) => self
				.load_if_present(format!("{xdg_config}/{authors_dir}/{file_path}"))
				.or_else(|| self.home_fallback(&authors_dir, &file_path)),
		}
	}
}
