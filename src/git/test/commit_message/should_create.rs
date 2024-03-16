use super::helper::*;
use crate::git::commit_message::CommitMessage;

fn test_cases() -> Vec<TestCase> {
	vec![
		TestCase {
			name: "happy path",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{A_LINE}"),
			authors: vec![A_LINE.to_string(), A_LINE.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{A_LINE}\n{A_LINE}"),
		},
		TestCase {
			name: "too many newlines",
			subject: A_SUBJECT.to_string(),
			body: format!("\n{A_LINE}\n\n\n{A_LINE}\n"),
			authors: vec![A_LINE.to_string(), A_LINE.to_string()],
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{A_LINE}\n{A_LINE}"),
		},
		TestCase {
			name: "no body",
			subject: A_SUBJECT.to_string(),
			body: EMPTY.to_string(),
			authors: vec![A_LINE.to_string(), A_LINE.to_string()],
			expected: format!("{A_SUBJECT}\n\n\n{A_LINE}\n{A_LINE}"),
		},
		TestCase {
			name: "no authors",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{A_LINE}"),
			authors: NO_AUTHORS.clone(),
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}"),
		},
		TestCase {
			name: "no body or authors",
			subject: A_SUBJECT.to_string(),
			body: EMPTY.to_string(),
			authors: NO_AUTHORS.clone(),
			expected: A_SUBJECT.to_string(),
		},
		TestCase {
			name: "only authors",
			subject: EMPTY.to_string(),
			body: EMPTY.to_string(),
			authors: vec![AN_AUTHOR.to_string(), AN_AUTHOR.to_string()],
			expected: format!("\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"),
		},
		TestCase {
			name: "empty",
			subject: EMPTY.to_string(),
			body: EMPTY.to_string(),
			authors: NO_AUTHORS.clone(),
			expected: EMPTY.to_string(),
		},
		TestCase {
			name: "comment",
			subject: A_SUBJECT.to_string(),
			body: format!("{A_LINE}\n{COMMENT}\n{A_LINE}"),
			authors: NO_AUTHORS.clone(),
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{COMMENT}\n{A_LINE}"),
		},
		TestCase {
			name: "whitespaces",
			subject: format!("{A_SUBJECT}{WHITESPACE}"),
			body: format!("{A_LINE}{WHITESPACE}\n{A_LINE}"),
			authors: NO_AUTHORS.clone(),
			expected: format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}"),
		},
	]
}

#[test]
fn given_a_correctly_formatted_commit_message() {
	for case in test_cases() {
		let actual =
			CommitMessage::new(&format!("{}\n\n{}", case.subject, case.body), case.authors.clone()).to_string();
		assert_eq!(case.expected, actual, "{}", case.name);
	}
}

#[test]
fn given_a_poorly_formatted_commit_message() {
	for case in test_cases() {
		let actual = CommitMessage::new(&format!("{}\n{}", case.subject, case.body), case.authors).to_string();
		assert_eq!(case.expected, actual, "{}", case.name);
	}
}
