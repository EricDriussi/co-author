use crate::authors::author::Author;
use parameterized::parameterized;

fn setup_author() -> Author {
	let alias = "a";
	let name = "alice";
	let email = "alice@wonderland.not";
	Author::new(alias, name, email)
}

#[test]
fn should_present_the_correct_signature() {
	let author = setup_author();
	assert_eq!(author.signature(), "Co-authored-by: alice <alice@wonderland.not>");
}

#[test]
fn should_provide_an_alias_getter() {
	let author = setup_author();
	assert_eq!(author.alias(), "a");
}

#[test]
fn should_provide_a_name_getter() {
	let author = setup_author();
	assert_eq!(author.name(), "alice");
}

#[test]
fn should_be_equal_to_another_author_with_equal_data() {
	let author = Author::new("a", "alice", "alice@wonderland.not");
	let same_author = Author::new("a", "alice", "alice@wonderland.not");
	assert_eq!(author, same_author)
}

#[parameterized(different_author = {
	Author::new("b", "alice", "alice@wonderland.not"),
	Author::new("a", "not_alice", "alice@wonderland.not"),
	Author::new("a", "alice", "someone@wonderland.not")
})]
fn should_not_be_equal_to_another_author_with_different_data(different_author: Author) {
	let author = Author::new("a", "alice", "alice@wonderland.not");
	assert_ne!(author, different_author)
}
