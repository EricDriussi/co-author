use crate::authors::author::Author;

use super::{input_reader::MockReader, prompt::Prompt};

#[test]
fn should_prompt_for_commit_message() {
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter your commit message"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(reader);

	let _ = cli.prompt_commit_message();
	// Only interested in params passed to the mock (withf)
}

#[test]
fn should_trim_commit_message() {
	let trimmed_msg = "test commit message";
	let padded_msg = format!(" {trimmed_msg}  ");
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(padded_msg.clone()));
	let mut cli = Prompt::new(reader);

	let result = cli.prompt_commit_message();

	assert!(matches!(result, Ok(msg) if msg == trimmed_msg));
}

#[test]
fn should_prompt_for_aliases() {
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(|prompt_msg| prompt_msg.contains("Enter co-authors aliases"))
		.times(1)
		.returning(|_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(reader);

	let _ = cli.prompt_aliases(&[]);
	// Only interested in params passed to the mock (withf)
}

#[test]
fn should_pretty_print_authors_when_prompting_for_aliases() {
	let alias = "a";
	let name = "alice";
	let author = Author::from(alias, name, "email");
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.withf(move |prompt_msg| prompt_msg.contains(format!("⦔ {alias} -> {name}").as_str()))
		.times(1)
		.returning(move |_| Ok("whatever".to_string()));
	let mut cli = Prompt::new(reader);

	let _ = cli.prompt_aliases(&[author]);
	// Only interested in params passed to the mock (withf())
}

#[test]
fn should_space_split_aliases() {
	let aliases = " a b cd   ";
	let mut reader = MockReader::new();
	reader
		.expect_readline()
		.times(1)
		.returning(move |_| Ok(aliases.to_string()));
	let mut cli = Prompt::new(reader);

	let result = cli.prompt_aliases(&[]);

	assert!(matches!(result, Ok(aliases) if aliases == ["a", "b", "cd"]));
}
