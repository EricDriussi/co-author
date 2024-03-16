use super::author::AuthorsProvider;
use super::csv::provider::CSVReader;
use super::load_mode::LoadMode;
use crate::common::fs::wrapper::FsWrapper;
use crate::Result;

pub fn init_authors_module_for(file: &str) -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromPath {
		file_loader: &FsWrapper::new(),
		path: file,
	})?))
}

pub fn init_authors_module() -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::load(&LoadMode::FromCwd {
		file_loader: &FsWrapper::new(),
	})?))
}
