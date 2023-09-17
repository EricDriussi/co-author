use std::error::Error;

#[derive(Debug)]
pub struct CliError(String);

impl std::fmt::Display for CliError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "CLI failed: {}", self.0)
	}
}

impl Error for CliError {}
impl CliError {
	pub fn new(msg: &str) -> Box<dyn Error> {
		Box::new(CliError(msg.to_string()))
	}
}
