use super::{cli::Cli, input_reader::MockInputReader};
use crate::{authors::author::Author, common::runner::MockRunner};

#[test]
fn prompt_for_message() {
	let mut reader = MockInputReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter commit message"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Cli::new(Box::new(reader), Box::new(MockRunner::new()));

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
	let mut cli = Cli::new(Box::new(reader), Box::new(MockRunner::new()));

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
	let mut cli = Cli::new(Box::new(reader), Box::new(MockRunner::new()));

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
	let mut cli = Cli::new(Box::new(reader), Box::new(MockRunner::new()));

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
	let mut cli = Cli::new(Box::new(reader), Box::new(MockRunner::new()));

	let _ = cli.aliases_prompt(&[author]);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn prompt_for_aliases_using_fzf() {
	let mut runner = MockRunner::new();
	runner
		.expect_attach()
		.withf(|cmd, args| cmd == "fzf" && args == ["--multi".to_string(), "--ansi".to_string()])
		.times(1)
		.returning(|_, _| Err("irrelevant".into())); // This is done to avoid creating a Child
	let cli = Cli::new(Box::new(MockInputReader::new()), Box::new(runner));

	let _ = cli.fzf_prompt(&[]);
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
