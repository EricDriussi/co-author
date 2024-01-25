use super::{file::File, FileLoader, Readable};

pub struct FsWrapper {}

impl FileLoader for FsWrapper {
	fn load_file(&self, file_path: String) -> Option<Box<dyn Readable>> {
		File::from(file_path)
	}
}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}
