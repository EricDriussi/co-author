use super::author::AuthorsProvider;
use super::csv::reader::{CSVReader, LoadMode};
use crate::common::fs::wrapper::FsWrapper;
use crate::Result;

pub fn init_for(file: &str) -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromPath {
		file_loader: &FsWrapper::new(),
		path: file,
	})?))
}

pub fn init() -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromCwd {
		file_loader: &FsWrapper::new(),
	})?))
}
