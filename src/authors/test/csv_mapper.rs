use crate::authors::author::Author;
use crate::authors::csv_mapper::CsvMapper;

#[test]
fn should_map_from_valid_csv_line() {
	let alias = "a";
	let name = "alice";
	let email = "alice@wonderland.not";

	let author_from_csv = CsvMapper::to_author(format!("{},{},{}", alias, name, email).as_str());

	assert_eq!(author_from_csv, Some(Author::from(alias, name, email)));
}

#[test]
fn should_not_map_from_invalid_csv_line() {
	let name = "alice";
	let email = "alice@wonderland.not";

	let no_author = CsvMapper::to_author(format!("{},{}", name, email).as_str());

	assert_eq!(no_author, None);
}
