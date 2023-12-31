use std::error::Error;

use co_author::{args::Args, authors::author::Author, cli::Cli, handle_authors, handle_commit_msg};

#[test]
fn commit_message_is_gathered_from_arg() {
	let mut cli = MockCli::with_commit_msg("IRRELEVANT");
	let prev = "IRRELEVANT".to_string();

	let message_by_param = "a commit message";
	let args = Args {
		message: Some(message_by_param.to_string()),
		editor: false,
		pre_populate: false,
		file: None,
		list: None,
		all: false,
		sort: false,
	};

	let msg = handle_commit_msg(&args, &mut cli, prev);

	assert!(msg.is_ok());
	assert_eq!(msg.unwrap().to_string(), message_by_param.to_string());
}

#[test]
fn commit_message_is_gathered_from_cli_prompt() {
	let prev = "IRRELEVANT".to_string();
	let args = Args {
		message: None,
		editor: false,
		pre_populate: false,
		file: None,
		list: None,
		all: false,
		sort: false,
	};

	let message_by_prompt = "a commit message";
	let mut cli = MockCli::with_commit_msg(message_by_prompt);

	let msg = handle_commit_msg(&args, &mut cli, prev);

	assert!(msg.is_ok());
	assert_eq!(msg.unwrap().to_string(), message_by_prompt.to_string());
}

#[test]
fn commit_message_is_gathered_from_pre_populated_cli_prompt() {
	let prev = "last commit message".to_string();
	let args = Args {
		message: None,
		editor: false,
		pre_populate: true,
		file: None,
		list: None,
		all: false,
		sort: false,
	};

	let message_by_prompt = "a new commit message";
	let mut cli = MockCli::with_commit_msg(message_by_prompt);

	let msg = handle_commit_msg(&args, &mut cli, prev.clone());

	assert!(msg.is_ok());
	let full_message = format!("{}{}", prev, message_by_prompt);
	assert_eq!(msg.unwrap().to_string(), full_message);
}

#[test]
fn authors_signatures_are_gathered_from_list() {
	let mut cli = MockCli::with_aliases("IRRELEVANT");
	let args = Args {
		message: None,
		editor: false,
		pre_populate: false,
		file: None,
		list: Some("a,b,cd".to_string()),
		all: false,
		sort: false,
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
		pre_populate: false,
		file: None,
		list: None,
		all: false,
		sort: false,
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

	fn ask_for_commit_message_with_pre_populated(&mut self, prev: String) -> Result<String, Box<dyn Error>> {
		Ok(format!("{}{}", prev, self.commit_msg.clone()))
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
