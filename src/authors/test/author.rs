use crate::authors::author::Author;
use parameterized::parameterized;

#[test]
fn should_present_a_co_author_compliant_signature() {
	let name = "alice";
	let email = "alice@wonderland.not";
	let author = Author::from("a", name, email);

	assert_eq!(author.signature(), format!("Co-Authored-by: {name} <{email}>"));
}

#[test]
fn should_get_alias() {
	let alias = "a";
	let author = Author::from(alias, "alice", "alice@wonderland.not");

	assert_eq!(author.alias(), alias);
}

#[test]
fn should_get_name() {
	let name = "alice";
	let author = Author::from("a", name, "alice@wonderland.not");

	assert_eq!(author.name(), name);
}

#[test]
fn should_be_equal_to_another_author_with_equal_data() {
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
fn should_not_be_equal_to_another_author_with_different_data(different_author: Author) {
	assert_ne!(Author::from("a", "alice", "alice@wonderland.not"), different_author);
}
