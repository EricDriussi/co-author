use super::*;

#[test]
fn should_error_when_given_an_empty_commit_message() {
	let msg = "".to_string();
	let result = FancyCli::process_commit_msg(msg);
	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err().to_string(),
		"CLI failed: Commit message cannot be empty."
	);
}

#[test]
fn should_return_the_submitted_commit_message_if_not_empty() {
	let msg = "test commit message".to_string();
	let result = FancyCli::process_commit_msg(msg.clone());
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), msg);
}

#[test]
fn should_trim_the_commit_message() {
	let trimmed_msg = "test commit message".to_string();
	let msg = format!("{}  ", trimmed_msg.clone());
	let result = FancyCli::process_commit_msg(msg);
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), trimmed_msg);
}

#[test]
fn should_parse_a_list_of_aliases() {
	let input_aliases = "a b cd".to_string();
	let result = FancyCli::process_aliases(input_aliases);
	assert_eq!(result, Vec::from(["a", "b", "cd"]));
}

#[test]
fn should_parse_an_empty_list_of_aliases() {
	let test_cases = vec!["", " ", "    "];
	for case in test_cases {
		let input_aliases = case.to_string();
		let result = FancyCli::process_aliases(input_aliases);
		assert_eq!(result, Vec::<String>::new());
	}
}

#[test]
fn should_format_authors_for_prompt() {
	let author = Author::new("alias", "name", "email");
	let prompt = FancyCli::format_author(&author);
	assert_eq!(strip_ansi::strip_ansi(prompt.as_str()), "⦔ alias -> name");
}
