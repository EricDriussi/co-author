use std::io::Write;
use std::io::{BufRead, BufReader, Error};
use std::iter::MapWhile;

type Lines = Vec<String>;

pub trait Readable {
	fn non_empty_lines(&self) -> Lines;
}

pub trait Writable {
	fn write(&mut self, data: String) -> crate::Result<()>;
}

pub trait Locatable {
	fn path(&self) -> &str;
}

pub trait File: Readable + Writable + Locatable {}

pub struct SimpleFile {
	file: std::fs::File,
	path: String,
}

impl SimpleFile {
	pub fn from(file: std::fs::File, path: String) -> Box<dyn File> {
		Box::new(Self { file, path })
	}

	fn valid_lines(&self) -> FSLines {
		BufReader::new(&self.file).lines().map_while(Result::ok)
	}
}

impl Readable for SimpleFile {
	fn non_empty_lines(&self) -> Lines {
		self.valid_lines().filter(|line| !line.trim().is_empty()).collect()
	}
}

impl Writable for SimpleFile {
	fn write(&mut self, data: String) -> crate::Result<()> {
		self.file.write_all(data.as_bytes())?;
		Ok(())
	}
}

impl Locatable for SimpleFile {
	fn path(&self) -> &str {
		self.path.as_str()
	}
}

impl File for SimpleFile {}

type FSLines<'a> = MapWhile<std::io::Lines<BufReader<&'a std::fs::File>>, fn(Result<String, Error>) -> Option<String>>;
