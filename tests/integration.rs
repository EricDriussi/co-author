use std::io::{sink, Cursor};

use authors::{
	app_service::AuthorsService,
	author::{Author, AuthorsRepo},
};
use co_author::{args::Args, cli::Cli, get_authors_signatures, get_commit_message, run_interactive};
use git::{
	app_service::GitService,
	git::{CommitBody, GitRepo},
};

// FIXME.rm this and re-think integration testing
#[test]
fn mocked_cli_flow() {
	let git_service = GitService::new(MockGitRepo::new());
	let authors_service = AuthorsService::new(MockAuthorRepo::new());
	let nothing = sink();
	let raw_input = Cursor::new(format!("{}{}{}{}", "a commit message", "\n", "a b cd", "\n"));
	let cli = Cli::new(raw_input, nothing);

	let result = run_interactive(git_service, authors_service, cli);

	assert!(result.is_ok());
}

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

//TODO.Add test for getting message from editor

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

struct MockGitRepo {}

impl MockGitRepo {
	fn new() -> Self {
		Self {}
	}
}

impl GitRepo for MockGitRepo {
	fn commit(&self, _body: CommitBody) -> Result<(), String> {
		return Ok(());
	}
}

struct MockAuthorRepo {}

impl MockAuthorRepo {
	fn new() -> Self {
		Self {}
	}
}

impl AuthorsRepo for MockAuthorRepo {
	fn find(&self, _aliases: Vec<String>) -> Vec<Author> {
		return Vec::from([Author::new("a", "John", "Doe"), Author::new("b", "Jane", "Smith")]);
	}

	fn all(&self) -> Vec<Author> {
		return Vec::from([Author::new("a", "John", "Doe"), Author::new("b", "Jane", "Smith")]);
	}
}
