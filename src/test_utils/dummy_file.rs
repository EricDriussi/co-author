use crate::common::fs::file::{File, Locatable, Readable, Writable};
use crate::Result;

pub struct DummyFile {
	content: Vec<String>,
	path: String,
}

impl DummyFile {
	pub fn empty() -> Self {
		Self {
			content: (vec![]),
			path: String::new(),
		}
	}
	pub fn with(content: Vec<&str>) -> Self {
		Self {
			content: content.into_iter().map(String::from).collect(),
			path: String::new(),
		}
	}
}

impl File for DummyFile {}

impl Readable for DummyFile {
	fn non_empty_lines(&self) -> Vec<String> {
		self.content.clone()
	}
}

impl Writable for DummyFile {
	fn write(&mut self, _data: String) -> Result<()> {
		Ok(())
	}
}

impl Locatable for DummyFile {
	fn path(&self) -> &str {
		self.path.as_str()
	}
}
