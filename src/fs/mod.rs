pub mod file;
pub mod wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn load_file(&self, file_path: String) -> Option<Box<dyn Readable>>;
}

pub trait Readable {
	fn non_empty_lines(&self) -> Vec<String>;
	fn all_lines(&self) -> Vec<String>;
}

#[cfg(test)]
mod test;
