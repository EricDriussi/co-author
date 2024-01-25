use super::{file::File, FileLoader, Readable};

pub struct FsWrapper {}

impl FileLoader for FsWrapper {
	fn file_in_abs_path(&self, path: String) -> Option<Box<dyn Readable>> {
		File::from(path)
	}

	fn file_in_cwd(&self, file_name: String) -> Option<Box<dyn Readable>> {
		File::from(std::env::current_dir().ok()?.join(file_name).display().to_string())
	}
}

impl FsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}
