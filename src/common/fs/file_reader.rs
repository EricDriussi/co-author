use crate::{common::err::SystemError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub type Lines = Vec<String>;

#[cfg_attr(test, mockall::automock)]
pub trait Reader {
	fn read_non_empty_lines(&self, path: &Path) -> Result<Lines>;
}

pub struct FileReader;

impl FileReader {
	pub fn new() -> FileReader {
		FileReader
	}
}

impl Reader for FileReader {
	fn read_non_empty_lines(&self, path: &Path) -> Result<Lines> {
		Ok(
			BufReader::new(File::open(path).map_err(|e| SystemError::Read(e.to_string()))?)
				.lines()
				.map_while(core::result::Result::ok)
				.filter(|line| !line.trim().is_empty())
				.collect(),
		)
	}
}
