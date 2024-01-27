use super::file::{File, OptionalReadable};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn load_file(&self, file_path: String) -> OptionalReadable;
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
}
