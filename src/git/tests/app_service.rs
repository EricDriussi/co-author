use git::{
	app_service::GitService,
	git::{CommitBody, GitRepo},
};

#[test]
fn should_commit() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let commit_message = "something";
	let aliases = vec![String::from("a")];

	let result = service.commit(commit_message, aliases);

	assert!(result.is_ok());
}

#[test]
fn should_not_allow_editor_commit_with_no_message() {
	let spy = MockRepo::new();
	let service = GitService::new(spy);
	let aliases = vec![String::from("a")];

	let commit_editmsg_path = "../../.git/COMMIT_EDITMSG";
	std::fs::write(commit_editmsg_path, "").unwrap();
	std::env::set_var("EDITOR", "echo");

	let result = service.commit_with_editor(aliases);

	assert!(result.is_err());
}

struct MockRepo {}

impl MockRepo {
	fn new() -> Self {
		Self {}
	}
}

impl GitRepo for MockRepo {
	fn commit(&self, _body: CommitBody) -> Result<(), String> {
		return Ok(());
	}
}
