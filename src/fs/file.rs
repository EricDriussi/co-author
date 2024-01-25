use std::io::{BufRead, BufReader};

use super::Readable;

pub struct File {
	file: std::fs::File,
}

impl File {
	pub fn from(path: String) -> Option<Box<dyn Readable>> {
		let file = std::fs::File::open(path);
		match file {
			Ok(file) => Some(Box::new(Self { file })),
			Err(_) => None,
		}
	}
}

impl Readable for File {
	fn non_empty_lines(&self) -> Vec<String> {
		BufReader::new(&self.file)
			.lines()
			.map_while(Result::ok)
			.filter(|line| !line.trim().is_empty())
			.collect()
	}

	fn all_lines(&self) -> Vec<String> {
		BufReader::new(&self.file).lines().map_while(Result::ok).collect()
	}
}
