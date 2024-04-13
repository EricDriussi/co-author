use super::author::AuthorsProvider;
use super::csv::provider::{CSVProvider, LoadMode};
use crate::common::fs::file_reader::FileReader;
use crate::Result;
use std::path::PathBuf;

pub fn init(file: &Option<String>) -> Result<Box<dyn AuthorsProvider>> {
	let load_mode = match file {
		Some(file) => LoadMode::FromPath {
			file_reader: &FileReader,
			path: PathBuf::from(file),
		},
		None => LoadMode::FromCwd {
			file_reader: &FileReader,
		},
	};
	Ok(Box::new(CSVProvider::load(&load_mode)?))
}
