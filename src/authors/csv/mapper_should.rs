use crate::authors::{author::Author, csv::mapper};

#[test]
fn map_from_valid_csv_line() {
	let alias = "a";
	let name = "alice";
	let email = "alice@wonderland.not";

	let author_from_csv = mapper::to_author(format!("{alias},{name},{email}").as_str());

	assert_eq!(author_from_csv, Some(Author::from(alias, name, email)));
}

#[test]
fn not_map_from_invalid_csv_line() {
	let name = "alice";
	let email = "alice@wonderland.not";

	let no_author = mapper::to_author(format!("{name},{email}").as_str());

	assert_eq!(no_author, None);
}
