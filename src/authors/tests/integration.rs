use authors::{app_service::AuthorsService, fs_repo::FSRepo};

#[test]
fn authors_service_returns_stored_authors_signatures() {
	let valid_authors_file = conf::get_config().get::<String>("valid_test_authors_file").unwrap();
	let repo = FSRepo::from(Some(valid_authors_file)).unwrap();
	let app_service = AuthorsService::new(repo);

	let authors = app_service.signatures_of(Vec::from([String::from("a")]));

	assert_eq!(authors.len(), 1);
}

#[test]
fn authors_crate_setup_works_when_an_authors_file_is_found() {
	let valid_authors_file = conf::get_config().get::<String>("valid_test_authors_file").unwrap();

	assert!(authors::fs_setup(Some(valid_authors_file)).is_ok());
}

#[test]
fn authors_crate_fails_to_set_up_if_no_authors_file_is_found() {
	assert!(authors::fs_setup(Some("no_file_here".to_string())).is_err());
}
