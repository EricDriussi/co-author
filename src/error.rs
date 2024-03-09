use std::fmt::Display;

use crate::{authors::err::AuthorsError, cli::err::CliError};

#[derive(Debug)]
pub enum Error {
	Authors(AuthorsError),
	Cli(CliError),
}

impl std::error::Error for Error {}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Authors(err) => err.fmt(f),
			Error::Cli(err) => err.fmt(f),
		}
	}
}

impl From<AuthorsError> for Error {
	fn from(err: AuthorsError) -> Self {
		Error::Authors(err)
	}
}

impl From<CliError> for Error {
	fn from(err: CliError) -> Self {
		Error::Cli(err)
	}
}
