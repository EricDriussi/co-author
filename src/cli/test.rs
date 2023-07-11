use std::io::Cursor;

#[test]
fn should_return_the_submitted_commit_message_if_not_empty() {
    let expected_commit_message = "a commit message";
    let mut raw_input = Cursor::new(format!("{}{}", expected_commit_message, "\n"));

    let actual_commit_message = super::ask_for_commit_message(Some(&mut raw_input)).unwrap();

    assert_eq!(expected_commit_message, actual_commit_message);
}

#[test]
fn should_error_when_given_an_empty_commit_message() {
    let mut empty_input = Cursor::new("\n");

    let commit_message = super::ask_for_commit_message(Some(&mut empty_input));

    assert!(commit_message.is_err());
}

#[test]
fn should_return_the_submitted_author_aliases_as_vec() {
    let test_cases = vec![Vec::from(["a", "b", "cd", "efg"]), Vec::from([])];
    for case in test_cases {
        let aliases_list = case;
        let provided_aliases = aliases_list.join(" ");

        let mut raw_input = Cursor::new(format!("{}{}", provided_aliases, "\n"));

        let actual_aliases = super::ask_for_aliases(Some(&mut raw_input));

        assert_eq!(aliases_list, actual_aliases);
    }
}
