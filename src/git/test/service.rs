// use crate::fs::wrapper::MockFileLoader;
// use crate::git::commit_body;
// use crate::git::commit_body::GitWrapper;
// use crate::git::conf_provider::MockConfProvider;
// use crate::git::runner::MockRunner;
// use crate::git::service::GitService;
// use serial_test::serial;
// use std::error::Error;

// #[test]
// #[serial]
// fn should_commit() {
// 	let spy = MockWrapper::new();
// 	let service = GitService::new(spy, MockRunner::new(), MockFileLoader::new(), MockConfProvider::new());
// 	let commit_message = "something";
// 	let aliases = vec![String::from("a")];

// 	let result = service.commit(commit_message, aliases);

// 	assert!(result.is_ok());
// }

// #[test]
// fn should_return_last_commit_message_if_present() {
// 	let last_commit = "msg";
// 	let wrapper = MockWrapper::with_last_commit(last_commit.to_string());
// 	let service = GitService::new(
// 		wrapper,
// 		MockRunner::new(),
// 		MockFileLoader::new(),
// 		MockConfProvider::new(),
// 	);

// 	let last_msg = service.last_commit_message();

// 	assert_eq!(last_msg, last_commit);
// }

// #[test]
// fn should_return_empty_string_if_last_commit_is_not_present() {
// 	let wrapper = MockWrapper::with_last_commit_err();
// 	let service = GitService::new(
// 		wrapper,
// 		MockRunner::new(),
// 		MockFileLoader::new(),
// 		MockConfProvider::new(),
// 	);

// 	let last_msg_empty = service.last_commit_message();

// 	assert_eq!(last_msg_empty, "");
// }

// struct MockWrapper {
// 	last_commit: Result<String, Box<dyn Error>>,
// }

// impl MockWrapper {
// 	fn new() -> Self {
// 		Self {
// 			last_commit: Ok(String::new()),
// 		}
// 	}

// 	fn with_last_commit(msg: String) -> Self {
// 		Self { last_commit: Ok(msg) }
// 	}

// 	fn with_last_commit_err() -> Self {
// 		Self {
// 			last_commit: Err("err".into()),
// 		}
// 	}
// }

// impl GitWrapper for MockWrapper {
// 	fn commit(&self) -> Result<(), Box<dyn Error>> {
// 		Ok(())
// 	}

// 	fn write_to_editmsg(&self, _: &commit_body::CommitBody) -> Result<(), Box<dyn Error>> {
// 		Ok(())
// 	}

// 	fn add_status_to_editmsg(&self) -> Result<(), Box<dyn Error>> {
// 		Ok(())
// 	}

// 	fn prev_commit_msg(&self) -> Result<String, Box<dyn Error>> {
// 		match &self.last_commit {
// 			Ok(msg) => Ok(msg.into()),
// 			Err(_) => Err("".into()),
// 		}
// 	}
// }
