use super::*;

#[test]
fn should_get_all_available_authors() {
    let author_service = Service::new(MockRepo::new());
    let actual_authors = author_service.get_available_authors();
    assert_eq!(actual_authors, MockRepo::hardcoded_authors());
}

#[test]
fn should_find_requested_authors() {
    let author_service = Service::new(MockRepo::new());
    let actual_authors = author_service.find_authors(Vec::from(["a"]));
    let expected_author = MockRepo::hardcoded_authors()[0].clone();
    assert!(actual_authors.len() == 1);
    assert!(actual_authors.contains(&expected_author));
}

struct MockRepo {}

impl MockRepo {
    fn new() -> Self {
        Self {}
    }

    fn hardcoded_authors() -> Vec<Author> {
        return Vec::from([
            Author::new("a", "John", "Doe"),
            Author::new("b", "Jane", "Smith"),
        ]);
    }
}

impl Repository for MockRepo {
    fn find_authors(&self, aliases: Vec<&str>) -> Vec<Author> {
        let mut authors = Vec::new();
        if aliases.contains(&"a") {
            authors.push(MockRepo::hardcoded_authors()[0].clone());
        }
        if aliases.contains(&"b") {
            authors.push(MockRepo::hardcoded_authors()[1].clone());
        }
        return authors;
    }

    fn all_authors(&self) -> Vec<Author> {
        return MockRepo::hardcoded_authors();
    }
}
