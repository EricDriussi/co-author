use super::helper::*;
use crate::git::commit_message::CommitMessage;

fn test_cases() -> Vec<TestCase> {
	vec![
		TestCase::build_for("happy path")
			.subject(A_SUBJECT)
			.body(A_LINE)
			.authors(&[AN_AUTHOR])
			.expected(&format!("{A_SUBJECT}\n\n{A_LINE}\n\n\n{AN_AUTHOR}"))
			.create(),
		TestCase::build_for("too many newlines")
			.subject(A_SUBJECT)
			.body(&format!("\n{A_LINE}\n\n\n{A_LINE}\n"))
			.authors(&[A_LINE, A_LINE])
			.expected(&format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}\n\n\n{A_LINE}\n{A_LINE}"))
			.create(),
		TestCase::build_for("no body")
			.subject(A_SUBJECT)
			.body("")
			.authors(&[A_LINE, A_LINE])
			.expected(&format!("{A_SUBJECT}\n\n\n{A_LINE}\n{A_LINE}"))
			.create(),
		TestCase::build_for("no authors")
			.subject(A_SUBJECT)
			.body(&format!("{A_LINE}\n{A_LINE}"))
			.authors(&[])
			.expected(&format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}"))
			.create(),
		TestCase::build_for("no body or authors")
			.subject(A_SUBJECT)
			.body("")
			.authors(&[])
			.expected(&A_SUBJECT)
			.create(),
		TestCase::build_for("only authors")
			.subject("")
			.body("")
			.authors(&[AN_AUTHOR, AN_AUTHOR])
			.expected(&format!("\n\n\n{AN_AUTHOR}\n{AN_AUTHOR}"))
			.create(),
		TestCase::build_for("empty")
			.subject("")
			.body("")
			.authors(&[])
			.expected("")
			.create(),
		TestCase::build_for("comment")
			.subject(A_SUBJECT)
			.body(&format!("{A_LINE}\n{COMMENT}\n{A_LINE}"))
			.authors(&[])
			.expected(&format!("{A_SUBJECT}\n\n{A_LINE}\n{COMMENT}\n{A_LINE}"))
			.create(),
		TestCase::build_for("whitespaces")
			.subject(&format!("{A_SUBJECT}{WHITESPACE}"))
			.body(&format!("{A_LINE}{WHITESPACE}\n{A_LINE}"))
			.authors(&[])
			.expected(&format!("{A_SUBJECT}\n\n{A_LINE}\n{A_LINE}"))
			.create(),
	]
}

#[test]
fn given_a_correctly_formatted_commit_message() {
	for t in test_cases() {
		let actual = CommitMessage::new(
			&format!("{}\n\n{}", t.message.subject, t.message.body),
			t.message.authors.clone(),
		)
		.to_string();
		assert_eq!(t.expected, actual, "{}", t.name);
	}
}

#[test]
fn given_a_poorly_formatted_commit_message() {
	for t in test_cases() {
		let actual =
			CommitMessage::new(&format!("{}\n{}", t.message.subject, t.message.body), t.message.authors).to_string();
		assert_eq!(t.expected, actual, "{}", t.name);
	}
}
