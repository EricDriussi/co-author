use crate::authors::author::Author;

#[test]
fn should_present_the_correct_signature() {
	let alias = "a";
	let name = "alice";
	let email = "alice@wonderland.not";
	let author = Author::new(alias, name, email);

	assert_eq!(author.signature(), "Co-authored-by: alice <alice@wonderland.not>");
}
