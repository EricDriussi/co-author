use std::io::{sink, Cursor};

use super::*;

#[test]
fn should_return_the_submitted_commit_message_if_not_empty() {
	let nothing = sink();
	let expected_commit_message = "a commit message";
	let raw_input = Cursor::new(format!("{}{}", expected_commit_message, "\n"));
	let mut cli = Cli::new(raw_input, nothing);

	let actual_commit_message = cli.ask_for_commit_message().unwrap();

	assert_eq!(expected_commit_message, actual_commit_message);
}

#[test]
fn should_error_when_given_an_empty_commit_message() {
	let nothing = sink();
	let empty_input = Cursor::new("\n");
	let mut cli = Cli::new(empty_input, nothing);

	let commit_message = cli.ask_for_commit_message();

	assert!(commit_message.is_err());
}

#[test]
fn should_return_the_submitted_author_aliases_as_vec() {
	let test_cases = vec![Vec::from(["a", "b", "cd", "efg"]), Vec::from([])];
	for case in test_cases {
		let nothing = sink();

		let aliases_list = case;
		let provided_aliases = aliases_list.join(" ");
		let raw_input = Cursor::new(format!("{}{}", provided_aliases, "\n"));

		let mut cli = Cli::new(raw_input, nothing);

		let actual_aliases = cli.ask_for_aliases();

		assert_eq!(aliases_list, actual_aliases);
	}
}
