use authors::{app_service::AuthorsService, fs_repo::FSRepo};

#[test]
fn authors_service_returns_stored_authors_signatures() {
	let repo = FSRepo::from(Some("tests/data/authors".to_string())).unwrap();
	let app_service = AuthorsService::new(repo);

	let authors = app_service.signatures_of(Vec::from([String::from("a")]));

	assert_eq!(authors.len(), 1);
}
