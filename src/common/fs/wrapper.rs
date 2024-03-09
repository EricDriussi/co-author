use super::file::{File, SimpleFile};
use std::fs::OpenOptions;

pub type OptionalFile = Option<Box<dyn File>>;

#[cfg_attr(test, mockall::automock)]
pub trait FileLoader {
	fn load_if_present(&self, file_path: String) -> OptionalFile;
	fn load_or_create(&self, file_path: String) -> OptionalFile;
}

pub struct FsWrapper;

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
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

	fn load_or_create(&self, file_path: String) -> OptionalFile {
		let file = OpenOptions::new()
			.read(true)
			.append(true)
			.create(true)
			.open(file_path.clone())
			.ok()?;
		Some(SimpleFile::from(file, file_path))
	}
}
