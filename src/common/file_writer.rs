use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::Result;

#[cfg_attr(test, mockall::automock)]
pub trait Writer {
	fn overwrite(&self, path: &Path, content: String) -> Result<()>;
	fn append(&self, path: &Path, content: String) -> Result<()>;
}

pub struct SimpleWriter;

impl SimpleWriter {
	pub fn new() -> SimpleWriter {
		SimpleWriter
	}
}

impl Writer for SimpleWriter {
	fn overwrite(&self, path: &Path, content: String) -> Result<()> {
		let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
		file.write_all(content.as_bytes())?;
		file.flush().map_err(Into::into)
	}

	fn append(&self, path: &Path, content: String) -> Result<()> {
		let mut file = OpenOptions::new().write(true).append(true).open(path)?;
		file.write_all(content.as_bytes())?;
		file.flush().map_err(Into::into)
	}
}
