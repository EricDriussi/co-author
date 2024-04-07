use super::{cli::Cli, input_reader::MockInputReader};
use crate::authors::author::Author;

#[test]
fn prompt_for_message() {
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter your commit message"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Cli::new(Box::new(reader));

	let _ = cli.message_prompt();
	// Only interested in params passed to the mock (withf)
}

#[test]
fn trim_message() {
	let trimmed_msg = "test commit message";
	let padded_msg = format!(" {trimmed_msg}  ");
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(padded_msg.clone()));
	let mut cli = Cli::new(Box::new(reader));

	let result = cli.message_prompt();

	assert!(matches!(result, Ok(msg) if msg == trimmed_msg));
}

#[test]
fn prompt_for_aliases() {
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter co-authors aliases"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Cli::new(Box::new(reader));

	let _ = cli.aliases_prompt(&[]);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn space_split_aliases() {
	let aliases = " a b cd   ";
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(aliases.to_string()));
	let mut cli = Cli::new(Box::new(reader));

	let result = cli.aliases_prompt(&[]);

	assert!(matches!(result, Ok(aliases) if aliases == ["a", "b", "cd"]));
}

#[test]
fn pretty_print_authors_when_prompting_for_aliases() {
	let alias = "a";
	let name = "alice";
	let author = Author::from(alias, name, "email");
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.withf(move |prompt_msg| contains_in_order(prompt_msg, &["⦔", alias, "->", name]))
		.times(1)
		.returning(move |_| Ok("whatever".to_string()));
	let mut cli = Cli::new(Box::new(reader));

	let _ = cli.aliases_prompt(&[author]);
	// Only interested in params passed to the mock (withf)
}

fn contains_in_order(prompt_msg: &str, components: &[&str]) -> bool {
	// This is functionally equal to doing prompt_msg.contains("⦔ alias -> name")
	// but only considers presence and relative order, discarding "colored" formatting (bold, colors, etc.)
	let found: Vec<_> = components.iter().map(|comp| prompt_msg.find(comp)).collect();
	let are_contained = found.iter().all(|&comp| comp.is_some());
	let are_in_order = found.windows(2).all(|comp| comp[0] < comp[1]);
	are_contained && are_in_order
}
