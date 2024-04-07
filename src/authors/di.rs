use super::author::AuthorsProvider;
use super::csv::reader::{CSVReader, LoadMode};
use crate::common::fs::file_reader::SimpleReader;
use crate::Result;
use std::path::PathBuf;

pub fn init(file: Option<String>) -> Result<Box<dyn AuthorsProvider>> {
	let provider = match file {
		Some(file) => CSVReader::load(&LoadMode::FromPath {
			file_reader: &SimpleReader::new(),
			path: PathBuf::from(&file),
		})?,
		None => CSVReader::load(&LoadMode::FromCwd {
			file_reader: &SimpleReader::new(),
		})?,
	};
	Ok(Box::new(provider))
}
