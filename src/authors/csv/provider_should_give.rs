use super::provider::LoadMode;
use crate::authors::author::AuthorsProvider;
use crate::authors::csv::provider::CSVProvider;
use crate::common::fs::file_reader::MockReader;

#[test]
fn all_authors_in_file() {
	let provider = csv_provider_with(vec![
		"a,Name Surname,someone@users.noreply.github.com".to_string(),
		"b,username,something@gmail.com".to_string(),
	]);

	let retrieved_authors = provider.all();

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn only_author_matching_an_alias() {
	let provider = csv_provider_with(vec![
		"a,Name Surname,someone@users.noreply.github.com".to_string(),
		"b,username,something@gmail.com".to_string(),
	]);

	let retrieved_authors = provider.find(&["a".to_string()]);

	assert_eq!(retrieved_authors.len(), 1);
}

#[test]
fn all_authors_matching_an_alias() {
	let provider = csv_provider_with(vec![
		"a,Name Surname,someone@users.noreply.github.com".to_string(),
		"b,username,something@gmail.com".to_string(),
		"b,username2,something2@gmail.com".to_string(),
	]);

	let retrieved_authors = provider.find(&["b".to_string()]);

	assert_eq!(retrieved_authors.len(), 2);
}

#[test]
fn no_author_when_alias_doesnt_match() {
	let provider = csv_provider_with(vec!["a,Name Surname,someone@users.noreply.github.com".to_string()]);

	let retrieved_authors = provider.find(&["z".to_string()]);

	assert_eq!(retrieved_authors.len(), 0);
}

fn csv_provider_with(authors: Vec<String>) -> CSVProvider {
	let mut mock_file_reader = MockReader::new();
	mock_file_reader
		.expect_read_non_empty_lines()
		.times(1)
		.returning(move |_| Ok(authors.clone()));
	CSVProvider::load(&LoadMode::FromCwd {
		file_reader: &mock_file_reader,
	})
	.expect("Could not setup AuthorsProvider for test")
}
