use crate::git::commit_body::CommitBody;

#[test]
fn should_produce_a_correctly_formatted_commit_body() {
	let message = "Irrelevant commit message";
	let author1 = String::from("author 1");
	let author2 = String::from("author 2");
	let commit_body = CommitBody::new(message, Vec::from([author1.clone(), author2.clone()]));

	assert!(commit_body
		.to_string()
		.eq(&format!("{message}\n\n{author1}\n{author2}")));
}
