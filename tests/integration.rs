use std::io::{sink, Cursor};

use co_author::{args::Args, cli::Cli, get_authors_signatures, get_commit_message};

#[test]
fn commit_message_is_gathered_from_arg() {
	let nothing = sink();
	let raw_input = Cursor::new("");
	let cli = Cli::new(raw_input, nothing);

	let message_by_param = "a commit message";
	let args = Args {
		message: Some(message_by_param.to_string()),
		editor: false,
		file: None,
		list: None,
		all: false,
	};

	let result = get_commit_message(&args, cli);

	assert_eq!(result, Ok(message_by_param.to_string()));
}

#[test]
fn commit_message_is_gathered_from_cli_prompt() {
	let args = Args {
		message: None,
		editor: false,
		file: None,
		list: None,
		all: false,
	};

	let nothing = sink();
	let message_by_prompt = "a commit message";
	let raw_input = Cursor::new(message_by_prompt);
	let cli = Cli::new(raw_input, nothing);

	let result = get_commit_message(&args, cli);

	assert_eq!(result, Ok(message_by_prompt.to_string()));
}

#[test]
fn test_get_commit_from_editor() {
	let commit_editmsg_path = "./.git/COMMIT_EDITMSG";
	let test_commit_message = "Test commit message.\nThis is a second line.\n".to_string();
	std::fs::write(commit_editmsg_path, test_commit_message.clone()).unwrap();

	std::env::set_var("EDITOR", "echo");

	let args = Args {
		message: None,
		editor: true,
		file: None,
		list: None,
		all: false,
	};

	let nothing = sink();
	let no_input = Cursor::new("");
	let cli = Cli::new(no_input, nothing);

	let result = get_commit_message(&args, cli);

	assert_eq!(result, Ok(test_commit_message));

	// Cleanup
	std::fs::remove_file(commit_editmsg_path).unwrap();
}

#[test]
fn authors_signatures_are_gathered_from_list() {
	let nothing = sink();
	let raw_input = Cursor::new("");
	let cli = Cli::new(raw_input, nothing);

	let args = Args {
		message: None,
		editor: false,
		file: None,
		list: Some("a,b,cd".to_string()),
		all: false,
	};

	let signatures = get_authors_signatures(&args, cli);

	assert!(signatures.is_ok());
	assert_eq!(
		signatures.unwrap(),
		Vec::from([
			"Co-Authored-by: Name Surname <someone@users.noreply.github.com>",
			"Co-Authored-by: username <something@gmail.com>",
			"Co-Authored-by: username2 <something2@gmail.com>"
		])
	);
}

#[test]
fn authors_signatures_are_gathered_from_cli_prompt() {
	let args = Args {
		message: None,
		editor: false,
		file: None,
		list: None,
		all: false,
	};

	let nothing = sink();
	let raw_input = Cursor::new("a b cd");
	let cli = Cli::new(raw_input, nothing);

	let signatures = get_authors_signatures(&args, cli);

	assert!(signatures.is_ok());
	assert_eq!(
		signatures.unwrap(),
		Vec::from([
			"Co-Authored-by: Name Surname <someone@users.noreply.github.com>",
			"Co-Authored-by: username <something@gmail.com>",
			"Co-Authored-by: username2 <something2@gmail.com>"
		])
	);
}
