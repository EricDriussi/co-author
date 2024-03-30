use super::author::AuthorsProvider;
use super::csv::reader::{CSVReader, LoadMode};
use crate::common::fs::file_reader::SimpleReader;
use crate::Result;
use std::path::PathBuf;

pub fn init_for(file: &str) -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromPath {
		file_reader: &SimpleReader::new(),
		path: PathBuf::from(file),
	})?))
}

pub fn init() -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromCwd {
		file_reader: &SimpleReader::new(),
	})?))
}
