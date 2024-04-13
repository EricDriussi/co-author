use super::author::AuthorsProvider;
use super::csv::provider::{CSVProvider, LoadMode};
use crate::common::fs::file_reader::FileReader;
use crate::Result;
use std::path::PathBuf;

pub fn init(file: &Option<String>) -> Result<Box<dyn AuthorsProvider>> {
	let provider = match file {
		Some(file) => CSVProvider::load(&LoadMode::FromPath {
			file_reader: &FileReader::new(),
			path: PathBuf::from(&file),
		})?,
		None => CSVProvider::load(&LoadMode::FromCwd {
			file_reader: &FileReader::new(),
		})?,
	};
	Ok(Box::new(provider))
}
