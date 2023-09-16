use super::*;

#[test]
fn should_error_when_given_an_empty_commit_message() {
	let msg = "".to_string();
	let result = Cli::process_commit_msg(msg);
	assert_eq!(result, Err("Commit message cannot be empty."));
}

#[test]
fn should_return_the_submitted_commit_message_if_not_empty() {
	let msg = "test commit message".to_string();
	let result = Cli::process_commit_msg(msg.clone());
	assert_eq!(result, Ok(msg));
}

#[test]
fn should_trim_the_commit_message() {
	let trimmed_msg = "test commit message".to_string();
	let msg = format!("{}  ", trimmed_msg.clone());
	let result = Cli::process_commit_msg(msg);
	assert_eq!(result, Ok(trimmed_msg));
}

#[test]
fn should_parse_a_list_of_aliases() {
	let input_aliases = "a b cd".to_string();
	let result = Cli::process_aliases(input_aliases);
	assert_eq!(result, Vec::from(["a", "b", "cd"]));
}

#[test]
fn should_parse_an_empty_list_of_aliases() {
	let test_cases = vec!["", " ", "    "];
	for case in test_cases {
		let input_aliases = case.to_string();
		let result = Cli::process_aliases(input_aliases);
		assert_eq!(result, Vec::<String>::new());
	}
}
