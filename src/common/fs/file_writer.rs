use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::common::err::SystemError;
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
		file.write_all(content.as_bytes())
			.map_err(|e| SystemError::Write(e.to_string()))?;
		Ok(file.flush().map_err(|e| SystemError::Write(e.to_string()))?)
	}
}

impl Writer for SimpleWriter {
	fn overwrite(&self, path: &Path, content: &str) -> Result<()> {
		let file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(path)
			.map_err(|e| SystemError::Write(e.to_string()))?;
		Self::write(file, content)
	}

	fn append(&self, path: &Path, content: &str) -> Result<()> {
		let file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(path)
			.map_err(|e| SystemError::Write(e.to_string()))?;
		Self::write(file, content)
	}
}
