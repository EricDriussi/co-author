use std::io::{BufRead, BufReader};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait FsWrapper {
	fn file_in_abs_path(&self, path: String) -> Option<Box<dyn File>>;
	fn file_in_cwd(&self, file_name: String) -> Option<Box<dyn File>>;
}

pub struct SimpleFsWrapper {}

impl FsWrapper for SimpleFsWrapper {
	fn file_in_abs_path(&self, path: String) -> Option<Box<dyn File>> {
		let file = std::fs::File::open(path);
		match file {
			Ok(file) => Some(Box::new(CSVFile::new(file))),
			Err(_) => None,
		}
	}

	fn file_in_cwd(&self, file_name: String) -> Option<Box<dyn File>> {
		let file = std::fs::File::open(std::env::current_dir().ok()?.join(file_name));
		match file {
			Ok(file) => Some(Box::new(CSVFile::new(file))),
			Err(_) => None,
		}
	}
}

impl SimpleFsWrapper {
	pub fn new() -> Self {
		Self {}
	}
}

pub trait File {
	fn read_lines(&self) -> Vec<String>;
}

pub struct CSVFile {
	file: std::fs::File,
}

impl CSVFile {
	pub fn new(file: std::fs::File) -> Self {
		Self { file }
	}
}

impl File for CSVFile {
	fn read_lines(&self) -> Vec<String> {
		BufReader::new(&self.file).lines().map_while(Result::ok).collect()
	}
}
