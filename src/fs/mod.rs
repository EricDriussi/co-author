pub mod file;
pub mod wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn file_in_abs_path(&self, path: String) -> Option<Box<dyn Readable>>;
	fn file_in_cwd(&self, file_name: String) -> Option<Box<dyn Readable>>;
}

pub trait Readable {
	fn non_empty_lines(&self) -> Vec<String>;
	fn all_lines(&self) -> Vec<String>;
}

#[cfg(test)]
mod test;
