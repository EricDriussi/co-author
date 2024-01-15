use std::{
	error::Error,
	fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub struct AuthorError(String);

impl Display for AuthorError {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "AUTHORS: {}", self.0)
	}
}

impl Error for AuthorError {}
impl AuthorError {
	pub fn with(msg: String) -> Box<dyn Error> {
		Box::new(AuthorError(msg))
	}
}
