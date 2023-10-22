use serial_test::serial;

use crate::{
	authors::{self, fs_repo::FSRepo, service::AuthorsService},
	conf,
};

#[test]
fn authors_service_returns_stored_authors_signatures() {
	let repo = FSRepo::from("src/authors/test/data/dummy_data".to_string()).unwrap();
	let app_service = AuthorsService::new(repo);

	let authors = app_service.signatures_of(Vec::from([String::from("a")]));

	assert_eq!(authors.len(), 1);
}

#[test]
#[ignore]
fn authors_crate_setup_works_when_an_authors_file_is_found() {
	let valid_authors_file = conf::authors_file();

	assert!(authors::fs_setup_from_file(valid_authors_file).is_ok());
}

#[test]
#[serial]
fn authors_crate_fails_to_set_up_if_no_authors_file_is_found() {
	assert!(authors::fs_setup_from_file("no_file_here".to_string()).is_err());
}
