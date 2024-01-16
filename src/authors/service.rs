use super::author::{Author, AuthorsRepo};

pub struct AuthorsService<T: AuthorsRepo> {
	repo: T,
}

impl<T: AuthorsRepo> AuthorsService<T> {
	pub fn new(repo: T) -> Self {
		Self { repo }
	}

	pub fn all_authors(&self) -> Vec<Author> {
		self.repo.all()
	}

	pub fn signatures_of(&self, aliases: Vec<String>) -> Vec<String> {
		self.repo
			.find(aliases)
			.iter()
			.map(|author| author.signature())
			.collect()
	}

	pub fn all_signatures(&self) -> Vec<String> {
		self.repo.all().iter().map(|author| author.signature()).collect()
	}
}
