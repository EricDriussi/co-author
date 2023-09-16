use authors::{fs_repo::FSRepo, service::AuthorsService};

#[test]
fn authors_service_returns_stored_authors_signatures() {
	let valid_authors_file = conf::get_config()
		.get::<String>("authors_file_from_authors_crate")
		.unwrap();
	let repo = FSRepo::from(valid_authors_file).unwrap();
	let app_service = AuthorsService::new(repo);

	let authors = app_service.signatures_of(Vec::from([String::from("a")]));

	assert_eq!(authors.len(), 1);
}

#[test]
fn authors_crate_setup_works_when_an_authors_file_is_found() {
	let valid_authors_file = conf::get_config()
		.get::<String>("authors_file_from_authors_crate")
		.unwrap();

	assert!(authors::fs_setup_from_file(valid_authors_file).is_ok());
}

#[test]
fn authors_crate_fails_to_set_up_if_no_authors_file_is_found() {
	assert!(authors::fs_setup_from_file("no_file_here".to_string()).is_err());
	assert!(authors::fs_default_setup("or_here".to_string()).is_err());
}
