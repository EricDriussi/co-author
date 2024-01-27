use std::io::{BufRead, BufReader, Error};
use std::iter::MapWhile;

type Lines = Vec<String>;

pub trait Readable {
	fn non_empty_lines(&self) -> Lines;
	fn all_lines(&self) -> Lines;
}

pub struct File {
	file: std::fs::File,
}

pub type OptionalReadable = Option<Box<dyn Readable>>;

impl File {
	pub fn from(path: String) -> OptionalReadable {
		let file = std::fs::File::open(path);
		match file {
			Ok(file) => Some(Box::new(Self { file })),
			Err(_) => None,
		}
	}

	fn valid_lines(&self) -> FSLines {
		BufReader::new(&self.file).lines().map_while(Result::ok)
	}
}

impl Readable for File {
	fn non_empty_lines(&self) -> Lines {
		self.valid_lines().filter(|line| !line.trim().is_empty()).collect()
	}

	fn all_lines(&self) -> Lines {
		self.valid_lines().collect()
	}
}

type FSLines<'a> = MapWhile<std::io::Lines<BufReader<&'a std::fs::File>>, fn(Result<String, Error>) -> Option<String>>;
