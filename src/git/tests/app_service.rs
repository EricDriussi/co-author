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
