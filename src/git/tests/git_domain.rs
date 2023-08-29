use git::git_domain::CommitBody;

#[test]
fn should_produce_a_correctly_formatted_commit_body() {
	let message = "Irrelevant commit message";
	let author1 = String::from("author 1");
	let author2 = String::from("author 2");
	let commit_body = CommitBody::new(message, Vec::from([author1.clone(), author2.clone()]));

	assert!(commit_body
		.to_string()
		.eq(&format!("{}\n\n{}\n{}", message, author1, author2)));
}
