use crate::authors::author::Author;

use super::{fancy_cli::FancyCli, input_reader::MockInputReader};

#[test]
fn should_prompt_for_commit_message() {
	let mut editor = MockInputReader::new();
	editor
		.expect_readline()
		.withf(|prompt| prompt.contains("Enter your commit message"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = FancyCli::new(editor);

	let _ = cli.prompt_commit_message();
	// Only interested in params passed to the mock (withf)
}

#[test]
fn should_trim_commit_message() {
	let trimmed_msg = "test commit message";
	let padded_msg = format!(" {trimmed_msg}  ");
	let mut editor = MockInputReader::new();
	editor
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(padded_msg.clone()));
	let mut cli = FancyCli::new(editor);

	let result = cli.prompt_commit_message();

	assert!(matches!(result, Ok(msg) if msg == trimmed_msg));
}

#[test]
fn should_prompt_for_aliases() {
	let mut editor = MockInputReader::new();
	editor
		.expect_readline()
		.withf(|prompt| prompt.contains("Enter co-authors aliases"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = FancyCli::new(editor);

	let _ = cli.prompt_aliases(&[]);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn should_pretty_print_authors_when_prompting_for_aliases() {
	let alias = "a";
	let name = "alice";
	let author = Author::from(alias, name, "email");
	let mut editor = MockInputReader::new();
	editor
		.expect_readline()
		.withf(move |prompt| prompt.contains(format!("⦔ {alias} -> {name}").as_str()))
		.times(1)
		.returning(move |_| Ok("whatever".to_string()));
	let mut cli = FancyCli::new(editor);

	let _ = cli.prompt_aliases(&[author]);
	// Only interested in params passed to the mock (withf())
}

#[test]
fn should_space_split_aliases() {
	let aliases = " a b cd   ";
	let mut editor = MockInputReader::new();
	editor
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(aliases.to_string()));
	let mut cli = FancyCli::new(editor);

	let result = cli.prompt_aliases(&[]);

	assert!(matches!(result, Ok(aliases) if aliases == ["a", "b", "cd"]));
}
