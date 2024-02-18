use crate::git::commit_message::CommitMessage;

#[test]
fn should_produce_a_correctly_formatted_commit_message() {
	let message = "Irrelevant commit message";
	let author1 = String::from("author 1");
	let author2 = String::from("author 2");
	let commit_message = CommitMessage::new(message, Vec::from([author1.clone(), author2.clone()]));

	assert!(commit_message
		.to_string()
		.eq(&format!("{message}\n\n{author1}\n{author2}")));
}
