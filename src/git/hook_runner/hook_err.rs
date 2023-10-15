use std::error::Error;

#[derive(Debug)]
pub struct HookError(String);

impl std::fmt::Display for HookError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} hook failed, aborting", self.0)
	}
}

impl Error for HookError {}
impl HookError {
	pub fn with(msg: &str) -> Box<dyn Error> {
		Box::new(HookError(msg.to_string()))
	}
}
