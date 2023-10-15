use std::error::Error;

#[derive(Debug)]
pub struct AuthorError(String);

impl std::fmt::Display for AuthorError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "AUTHORS: {}", self.0)
	}
}

impl Error for AuthorError {}
impl AuthorError {
	pub fn new(msg: String) -> Box<dyn Error> {
		Box::new(AuthorError(msg))
	}
}
