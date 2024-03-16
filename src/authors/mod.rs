pub mod author;
pub mod di;
pub mod err;
pub mod load_mode;

pub mod csv {
	mod mapper;
	pub mod provider;

	#[cfg(test)]
	mod mapper_should;
	#[cfg(test)]
	mod provider_should;
}

#[cfg(test)]
mod author_should;
