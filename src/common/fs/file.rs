use std::io::{BufRead, BufReader, Error};
use std::iter::MapWhile;
use std::{fs::OpenOptions, io::Write};

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

pub type OptionalFile = Option<Box<dyn File>>;

impl SimpleFile {
	pub fn from(path: String) -> OptionalFile {
		let file = std::fs::File::open(path.clone());
		Some(Box::new(Self { file: file.ok()?, path }))
	}

	pub fn open_or_create(path: String) -> OptionalFile {
		let file = OpenOptions::new()
			.read(true)
			.append(true)
			.create(true)
			.open(path.clone())
			.ok()?;
		Some(Box::new(Self { file, path }))
	}

	fn valid_lines(&self) -> FSLines {
		BufReader::new(&self.file).lines().map_while(Result::ok)
	}
}

impl File for SimpleFile {}

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

type FSLines<'a> = MapWhile<std::io::Lines<BufReader<&'a std::fs::File>>, fn(Result<String, Error>) -> Option<String>>;
