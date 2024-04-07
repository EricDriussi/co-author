use crate::{authors::author::Author, common::conf};
use parameterized::parameterized;

#[test]
fn present_a_co_author_compliant_signature() {
	let name = "alice";
	let email = "alice@wonderland.not";
	let author = Author::from("a", name, email);

	let co_author_prefix = conf::co_author_prefix();
	assert_eq!(author.signature(), format!("{co_author_prefix}: {name} <{email}>"));
}

#[test]
fn get_alias() {
	let alias = "a";
	let author = Author::from(alias, "alice", "alice@wonderland.not");

	assert_eq!(author.alias(), alias);
}

#[test]
fn get_name() {
	let name = "alice";
	let author = Author::from("a", name, "alice@wonderland.not");

	assert_eq!(author.name(), name);
}

#[test]
fn be_equal_to_another_author_with_same_data() {
	let alias = "a";
	let name = "alice";
	let email = "alice@wonderland.not";

	let author = Author::from(alias, name, email);
	let same_author = Author::from(alias, name, email);

	assert_eq!(author, same_author);
}

#[parameterized(different_author = {
	Author::from("b", "alice", "alice@wonderland.not"),
	Author::from("a", "not_alice", "alice@wonderland.not"),
	Author::from("a", "alice", "someone@wonderland.not")
})]
fn not_be_equal_to_another_author_with_different_data(different_author: Author) {
	assert_ne!(Author::from("a", "alice", "alice@wonderland.not"), different_author);
}
