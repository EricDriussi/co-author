use crate::common::fs::file::{File, Locatable, Readable, Writable};
use crate::Result;

#[derive(Clone)]
pub struct DummyFile {
	content: Vec<String>,
	path: String,
	write_was_called_with: String,
}

impl DummyFile {
	pub fn empty() -> Self {
		Self {
			content: (vec![]),
			path: String::new(),
			write_was_called_with: String::new(),
		}
	}

	pub fn with(content: Vec<&str>) -> Self {
		Self {
			content: content.into_iter().map(String::from).collect(),
			path: String::new(),
			write_was_called_with: String::new(),
		}
	}

	pub fn write_was_called_with(&self) -> &str {
		self.write_was_called_with.as_str()
	}
}

impl File for DummyFile {}

impl Readable for DummyFile {
	fn non_empty_lines(&self) -> Vec<String> {
		self.content.clone()
	}
}

impl Writable for DummyFile {
	fn write(&mut self, data: String) -> Result<()> {
		self.write_was_called_with = data;
		Ok(())
	}
}

impl Locatable for DummyFile {
	fn path(&self) -> &str {
		self.path.as_str()
	}
}
