use super::helper::*;
use crate::git::commit_message::CommitMessage;

fn test_cases() -> Vec<TestCase> {
	vec![
		TestCase {
			name: "happy path",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{A_LINE}"),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
		TestCase {
			name: "too many newlines",
			subject: A_SUBJECT.to_string(),
			body: format!("\n{A_LINE}\n\n\n{A_LINE}\n"),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
		TestCase {
			name: "no body",
			subject: A_SUBJECT.to_string(),
			body: EMPTY.to_string(),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("{A_SUBJECT}\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
		TestCase {
			name: "no authors",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{A_LINE}"),
			authors: vec![],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}"),
		},
		TestCase {
			name: "no body or authors",
			subject: A_SUBJECT.to_string(),
			body: EMPTY.to_string(),
			authors: vec![],
			expected: A_SUBJECT.to_string(),
		},
		TestCase {
			name: "empty",
			subject: EMPTY.to_string(),
			body: EMPTY.to_string(),
			authors: vec![],
			expected: EMPTY.to_string(),
		},
		TestCase {
			name: "comment",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{COMMENT}\n{A_LINE}"),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
		TestCase {
			name: "whitespaces",
			subject: format!("{A_SUBJECT}{WHITESPACE}"),
			body: format!("{A_LINE}{WHITESPACE}\n{A_LINE}"),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
	]
}

#[test]
fn a_correctly_formatted_commit_message() {
	for case in test_cases() {
		let actual = CommitMessage::from(&format!(
			"{}\n\n{}\n\n\n{}",
			case.subject,
			case.body,
			case.authors.join("\n")
		))
		.to_string();
		assert_eq!(case.expected, actual, "{}", case.name);
	}
}

#[test]
fn a_poorly_formatted_commit_message() {
	for case in test_cases() {
		let actual =
			CommitMessage::from(&format!("{}\n{}\n{}", case.subject, case.body, case.authors.join("\n"))).to_string();
		assert_eq!(case.expected, actual, "{}", case.name);
	}
}
