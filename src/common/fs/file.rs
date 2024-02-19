use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error};
use std::iter::MapWhile;

type Lines = Vec<String>;

pub trait Readable {
	fn non_empty_lines(&self) -> Lines;
	fn path(&self) -> &str;
}

pub struct File {
	file: std::fs::File,
	path: String,
}

pub type OptionalReadable = Option<Box<dyn Readable>>;

impl File {
	pub fn from(path: String) -> OptionalReadable {
		let file = std::fs::File::open(path.clone());
		Some(Box::new(Self { file: file.ok()?, path }))
	}

	pub fn open_or_create(path: String) -> OptionalReadable {
		let file = OpenOptions::new()
			.read(true)
			.write(true)
			.create(true)
			.open(path.clone())
			.ok()?;
		Some(Box::new(Self { file, path }))
	}

	fn valid_lines(&self) -> FSLines {
		BufReader::new(&self.file).lines().map_while(Result::ok)
	}
}

impl Readable for File {
	fn non_empty_lines(&self) -> Lines {
		self.valid_lines().filter(|line| !line.trim().is_empty()).collect()
	}

	fn path(&self) -> &str {
		self.path.as_str()
	}
}

type FSLines<'a> = MapWhile<std::io::Lines<BufReader<&'a std::fs::File>>, fn(Result<String, Error>) -> Option<String>>;
