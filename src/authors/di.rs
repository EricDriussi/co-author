use super::csv::provider::CSVReader;
use crate::common::fs::wrapper::FsWrapper;
use crate::Result;

pub fn init_authors_module_for(file: &str) -> Result<CSVReader> {
	CSVReader::from(&FsWrapper::new(), file)
}

pub fn init_authors_module() -> Result<CSVReader> {
	CSVReader::from_cwd_fallback_home(&FsWrapper::new())
}
