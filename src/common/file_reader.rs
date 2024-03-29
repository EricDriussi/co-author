use crate::Result;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::Path,
};

pub type Lines = Vec<String>;

#[cfg_attr(test, mockall::automock)]
pub trait Reader {
	fn read_non_empty_lines(&self, path: &Path) -> Result<Lines>;
}

pub struct SimpleReader;

impl SimpleReader {
	pub fn new() -> SimpleReader {
		SimpleReader
	}
}

impl Reader for SimpleReader {
	fn read_non_empty_lines(&self, path: &Path) -> Result<Lines> {
		Ok(BufReader::new(File::open(path)?)
			.lines()
			.map_while(core::result::Result::ok)
			.filter(|line| !line.trim().is_empty())
			.collect())
	}
}
