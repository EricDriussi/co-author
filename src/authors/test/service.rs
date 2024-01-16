use crate::authors::{
	author::{Author, AuthorsRepo},
	service::AuthorsService,
};

#[test]
fn should_find_requested_authors_signatures() {
	let author_service = AuthorsService::new(MockRepo::new());
	let actual_authors = author_service.signatures_of(Vec::from([String::from("a")]));
	let expected_signature = MockRepo::hardcoded_authors()[0].signature().clone();
	assert_eq!(actual_authors.len(), 1);
	assert!(actual_authors.contains(&expected_signature));
}

#[test]
fn should_find_all_authors_signatures() {
	let author_service = AuthorsService::new(MockRepo::new());
	let actual_authors = author_service.all_signatures();
	let expected_signature_1 = MockRepo::hardcoded_authors()[0].signature().clone();
	let expected_signature_2 = MockRepo::hardcoded_authors()[0].signature().clone();
	assert_eq!(actual_authors.len(), 2);
	assert!(actual_authors.contains(&expected_signature_1));
	assert!(actual_authors.contains(&expected_signature_2));
}

#[test]
fn should_get_all_available_authors() {
	let author_service = AuthorsService::new(MockRepo::new());
	let actual_authors = author_service.all_authors();
	assert_eq!(actual_authors, MockRepo::hardcoded_authors());
}

struct MockRepo {}

impl MockRepo {
	fn new() -> Self {
		Self {}
	}

	fn hardcoded_authors() -> Vec<Author> {
		Vec::from([Author::from("a", "John", "Doe"), Author::from("b", "Jane", "Smith")])
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
		authors
	}

	fn all(&self) -> Vec<Author> {
		MockRepo::hardcoded_authors()
	}
}
