use crate::common::fs::file::{File, Locatable, Readable, Writable};
use mockall::*;

mock! {
	pub File {}

	impl Readable for File {
		fn non_empty_lines(&self) -> Vec<String>;
	}

	impl Writable for File {
		fn write(&mut self, data: String) -> crate::Result<()>;
	}

	impl Locatable for File {
		fn path(&self) -> &str;
	}

	impl File for File {}
}
