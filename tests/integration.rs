use std::error::Error;

use authors::author::Author;
use co_author::{args::Args, cli::Cli, handle_authors, handle_commit_msg};

#[test]
fn commit_message_is_gathered_from_arg() {
	let mut cli = MockCli::with_commit_msg("IRRELEVANT");

	let message_by_param = "a commit message";
	let args = Args {
		message: Some(message_by_param.to_string()),
		editor: false,
		file: None,
		list: None,
		all: false,
	};

	let option = handle_commit_msg(&args, &mut cli);
	assert!(option.is_some());

	let result = option.unwrap();
	assert!(result.is_ok());
	assert_eq!(result.unwrap().to_string(), message_by_param.to_string());
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

	let message_by_prompt = "a commit message";
	let mut cli = MockCli::with_commit_msg(message_by_prompt);

	let option = handle_commit_msg(&args, &mut cli);
	assert!(option.is_some());

	let result = option.unwrap();
	assert!(result.is_ok());
	assert_eq!(result.unwrap().to_string(), message_by_prompt.to_string());
}

#[test]
fn commit_message_is_not_gathered_under_editor_flag() {
	let mut cli = MockCli::with_commit_msg("IRRELEVANT");
	let args = Args {
		message: None,
		editor: true,
		file: None,
		list: None,
		all: false,
	};

	let option = handle_commit_msg(&args, &mut cli);
	assert!(option.is_none());
}

#[test]
fn authors_signatures_are_gathered_from_list() {
	let mut cli = MockCli::with_aliases("IRRELEVANT");
	let args = Args {
		message: None,
		editor: false,
		file: None,
		list: Some("a,b,cd".to_string()),
		all: false,
	};

	let signatures = handle_authors(&args, &mut cli);

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

	let mut cli = MockCli::with_aliases("a b cd");
	let signatures = handle_authors(&args, &mut cli);

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

pub struct MockCli {
	commit_msg: String,
	aliases: Vec<String>,
}

impl Cli for MockCli {
	fn ask_for_commit_message(&mut self) -> Result<String, Box<dyn Error>> {
		Ok(self.commit_msg.clone())
	}

	fn ask_for_commit_message_with_prev(&mut self, _: String) -> Result<String, Box<dyn Error>> {
		Ok(self.commit_msg.clone())
	}

	fn ask_for_aliases(&mut self, _: Vec<Author>) -> Result<Vec<String>, Box<dyn Error>> {
		Ok(self.aliases.clone())
	}
}

impl MockCli {
	fn with_commit_msg(commit_msg: &str) -> Self {
		Self {
			commit_msg: commit_msg.to_string(),
			aliases: Vec::new(),
		}
	}

	fn with_aliases(aliases: &str) -> Self {
		Self {
			commit_msg: "".to_string(),
			aliases: aliases.split_whitespace().map(|s| s.to_string()).collect(),
		}
	}
}
