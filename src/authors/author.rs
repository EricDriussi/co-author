use crate::common::conf;

#[derive(Debug, PartialEq)]
pub struct Author {
	alias: String,
	name: String,
	email: String,
}

impl Author {
	pub fn from(alias: &str, name: &str, email: &str) -> Self {
		Self {
			alias: String::from(alias),
			name: String::from(name),
			email: String::from(email),
		}
	}

	pub fn signature(&self) -> String {
		format!("{}: {} <{}>", conf::co_author_prefix(), self.name, self.email)
	}

	pub fn alias(&self) -> String {
		self.alias.clone()
	}

	pub fn name(&self) -> String {
		self.name.clone()
	}
}

pub trait AuthorsProvider {
	fn find(&self, aliases: &[String]) -> Vec<Author>;
	fn all(&self) -> Vec<Author>;
}
