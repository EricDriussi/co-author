pub mod file;
pub mod wrapper;

type Lines = Vec<String>;
type OptionalReadable = Option<Box<dyn Readable>>;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait FileLoader {
	fn load_file(&self, file_path: String) -> OptionalReadable;
}

pub trait Readable {
	fn non_empty_lines(&self) -> Lines;
	fn all_lines(&self) -> Lines;
}

#[cfg(test)]
mod test;
