use std::fs;

pub struct AfterAssert {
	files: Vec<String>,
}

impl AfterAssert {
	pub fn cleanup_files(files: &[&str]) -> Self {
		Self {
			files: files.iter().map(ToString::to_string).collect(),
		}
	}

	pub fn cleanup_file(file: &str) -> Self {
		Self {
			files: vec![file.to_string()],
		}
	}
}

impl Drop for AfterAssert {
	fn drop(&mut self) {
		for file in &self.files {
			fs::remove_file(file).expect("Could not cleanup files");
		}
	}
}
