use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::Result;

#[cfg_attr(test, mockall::automock)]
pub trait Writer {
	fn overwrite(&self, path: &Path, content: &str) -> Result<()>;
	fn append(&self, path: &Path, content: &str) -> Result<()>;
}

pub struct SimpleWriter;

impl SimpleWriter {
	pub fn new() -> SimpleWriter {
		SimpleWriter
	}

	fn write(mut file: File, content: &str) -> Result<()> {
		file.write_all(content.as_bytes())?;
		file.flush().map_err(Into::into)
	}
}

impl Writer for SimpleWriter {
	fn overwrite(&self, path: &Path, content: &str) -> Result<()> {
		let file = OpenOptions::new().write(true).truncate(true).create(true).open(path)?;
		Self::write(file, content)
	}

	fn append(&self, path: &Path, content: &str) -> Result<()> {
		let file = OpenOptions::new().write(true).append(true).open(path)?;
		Self::write(file, content)
	}
}
