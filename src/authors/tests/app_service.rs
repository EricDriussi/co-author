use authors::{
	app_service::AuthorsService,
	author::{Author, AuthorsRepo},
};

#[test]
fn should_find_requested_authors() {
	let author_service = AuthorsService::new(MockRepo::new());
	let actual_authors = author_service.signatures_of(Vec::from([String::from("a")]));
	let expected_signature = MockRepo::hardcoded_authors()[0].signature().clone();
	assert_eq!(actual_authors.len(), 1);
	assert!(actual_authors.contains(&expected_signature));
}

#[test]
fn should_get_all_available_authors() {
	let author_service = AuthorsService::new(MockRepo::new());
	let actual_authors = author_service.all_available();
	assert_eq!(actual_authors, MockRepo::hardcoded_authors());
}

struct MockRepo {}

impl MockRepo {
	fn new() -> Self {
		Self {}
	}

	fn hardcoded_authors() -> Vec<Author> {
		return Vec::from([Author::new("a", "John", "Doe"), Author::new("b", "Jane", "Smith")]);
	}
}

impl AuthorsRepo for MockRepo {
	fn find(&self, aliases: Vec<String>) -> Vec<Author> {
		let mut authors = Vec::new();
		if aliases.contains(&"a".to_string()) {
			authors.push(MockRepo::hardcoded_authors()[0].clone());
		}
		if aliases.contains(&"b".to_string()) {
			authors.push(MockRepo::hardcoded_authors()[1].clone());
		}
		return authors;
	}

	fn all(&self) -> Vec<Author> {
		return MockRepo::hardcoded_authors();
	}
}
