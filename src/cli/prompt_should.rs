use super::{input_reader::MockReader, prompt::Prompt};
use crate::authors::author::Author;

#[test]
fn prompt_for_commit_message() {
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter your commit message"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(Box::new(reader));

	let _ = cli.prompt_commit_message();
	// Only interested in params passed to the mock (withf)
}

#[test]
fn trim_commit_message() {
	let trimmed_msg = "test commit message";
	let padded_msg = format!(" {trimmed_msg}  ");
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(padded_msg.clone()));
	let mut cli = Prompt::new(Box::new(reader));

	let result = cli.prompt_commit_message();

	assert!(matches!(result, Ok(msg) if msg == trimmed_msg));
}

#[test]
fn prompt_for_aliases() {
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter co-authors aliases"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(Box::new(reader));

	let _ = cli.prompt_aliases(&[]);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn pretty_print_authors_when_prompting_for_aliases() {
	let alias = "a";
	let name = "alice";
	let author = Author::from(alias, name, "email");
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(move |prompt_msg| contains_in_order(prompt_msg, &["⦔", alias, "->", name]))
		.times(1)
		.returning(move |_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(Box::new(reader));

	let _ = cli.prompt_aliases(&[author]);
	// Only interested in params passed to the mock (withf())
}

#[test]
fn space_split_aliases() {
	let aliases = " a b cd   ";
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(aliases.to_string()));
	let mut cli = Prompt::new(Box::new(reader));

	let result = cli.prompt_aliases(&[]);

	assert!(matches!(result, Ok(aliases) if aliases == ["a", "b", "cd"]));
}

fn contains_in_order(prompt_msg: &str, components: &[&str]) -> bool {
	// This is functionally equal to doing prompt_msg.contains("⦔ alias -> name")
	// but only considers presence and relative order, discarding "colored" formatting (bold, colors, etc.)
	let found: Vec<_> = components.iter().map(|comp| prompt_msg.find(comp)).collect();
	let are_contained = found.iter().all(|&comp| comp.is_some());
	let are_in_order = found.windows(2).all(|comp| comp[0] < comp[1]);
	are_contained && are_in_order
}
