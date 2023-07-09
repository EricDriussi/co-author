use std::io::Cursor;

#[test]
fn should_return_the_given_input_without_a_line_break() {
    let expected_input = "Ghostbusters!!";
    let mut raw_input = Cursor::new(format!("{}{}", expected_input, "\n"));

    let actual_input = super::prompt_user("Who you gonna call?", Some(&mut raw_input));

    assert_eq!(expected_input, actual_input);
    assert!(!actual_input.contains("\n"));
}
