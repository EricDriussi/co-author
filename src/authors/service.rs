use super::author::{Author, AuthorsProvider};

pub struct AuthorsService<T: AuthorsProvider> {
	repo: T,
}

impl<T: AuthorsProvider> AuthorsService<T> {
	pub fn new(repo: T) -> Self {
		Self { repo }
	}

	pub fn all_authors(&self) -> Vec<Author> {
		self.repo.all()
	}

	pub fn signatures_of(&self, aliases: Vec<String>) -> Vec<String> {
		self.repo.find(aliases).iter().map(Author::signature).collect()
	}

	pub fn all_signatures(&self) -> Vec<String> {
		self.repo.all().iter().map(Author::signature).collect()
	}
}
