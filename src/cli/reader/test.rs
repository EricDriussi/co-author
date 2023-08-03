use std::io;

#[test]
fn should_return_the_given_input_without_a_line_break() {
	let expected_input = "Ghostbusters!!";
	let nothing = io::sink();
	let fake_input = io::Cursor::new(format!("{}{}", expected_input, "\n"));

	let actual_input = super::prompt("Who you gonna call?", fake_input, nothing);

	assert_eq!(expected_input, actual_input);
	assert!(!actual_input.contains("\n"));
}
