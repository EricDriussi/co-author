pub mod author;
pub mod di;
pub mod err;

pub mod csv {
	mod mapper;
	pub mod reader;

	#[cfg(test)]
	mod mapper_should;
	#[cfg(test)]
	mod reader_should;
}

#[cfg(test)]
mod author_should;
