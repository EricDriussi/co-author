use std::io::Cursor;

#[test]
fn should_error_when_given_an_empty_commit_message() {
    let mut empty_input = Cursor::new("\n");

    let commit_message_result = super::ask_for_commit_message(Some(&mut empty_input));

    assert!(commit_message_result.is_err());
}

#[test]
fn should_return_the_submitted_commit_message() {
    let expected_input = "a commit message";
    let mut raw_input = Cursor::new(format!("{}{}", expected_input, "\n"));

    let commit_message_result = super::ask_for_commit_message(Some(&mut raw_input)).unwrap();

    assert_eq!(expected_input, commit_message_result);
}
