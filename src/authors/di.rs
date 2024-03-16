use super::author::AuthorsProvider;
use super::csv::provider::CSVReader;
use crate::common::fs::wrapper::FsWrapper;
use crate::Result;

pub fn init_authors_module_for(file: &str) -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::from(&FsWrapper::new(), file)?))
}

pub fn init_authors_module() -> Result<Box<dyn AuthorsProvider>> {
	Ok(Box::new(CSVReader::from_cwd_fallback_home(&FsWrapper::new())?))
}
