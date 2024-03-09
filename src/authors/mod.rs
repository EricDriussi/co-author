pub mod author;
pub mod err;

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
