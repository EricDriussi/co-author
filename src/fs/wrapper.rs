use super::{file::File, FileLoader, OptionalReadable};

pub struct FsWrapper {}

impl FileLoader for FsWrapper {
	fn load_file(&self, file_path: String) -> OptionalReadable {
		File::from(file_path)
	}
}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}
