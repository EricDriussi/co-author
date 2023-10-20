
#[derive(Debug, Clone)]
pub struct Author {
	pub alias: String,
	pub name: String,
	email: String,
}

impl Author {
	pub fn new(alias: &str, name: &str, email: &str) -> Self {
		Self {
			alias: String::from(alias),
			name: String::from(name),
			email: String::from(email),
		}
	}

	pub fn signature(&self) -> String {
		format!("Co-authored-by: {} <{}>", self.name, self.email)
	}
}

impl PartialEq for Author {
	fn eq(&self, other: &Self) -> bool {
		let same_alias = self.alias == other.alias;
		let same_name = self.name == other.name;
		let same_email = self.email == other.email;
		same_alias && same_name && same_email
	}
}

pub trait AuthorsRepo {
	fn find(&self, aliases: Vec<String>) -> Vec<Author>;
	fn all(&self) -> Vec<Author>;
}
