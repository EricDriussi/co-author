use std::fs;

pub struct AfterAssert {
	files: Vec<String>,
}

impl AfterAssert {
	pub fn cleanup(files: &[&str]) -> Self {
		Self {
			files: files.iter().map(|f| f.to_string()).collect(),
		}
	}
}

impl Drop for AfterAssert {
	fn drop(&mut self) {
		for file in &self.files {
			fs::remove_file(file).unwrap()
		}
	}
}
