use crate::common::conf;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

	pub fn hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		let to_hash = format!("{}{}", self.alias, self.name);
		to_hash.hash(&mut hasher);
		hasher.finish()
	}
}

pub trait AuthorsProvider {
	fn find_by_aliases(&self, aliases: &[String]) -> Vec<Author>;
	fn find_by_hashes(&self, hashes: &[u64]) -> Vec<Author>;
	fn all(&self) -> Vec<Author>;
}
