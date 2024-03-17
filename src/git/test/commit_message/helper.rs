pub const A_SUBJECT: &str = "subject line";
pub const A_LINE: &str = "line";
pub const AN_AUTHOR: &str = "Co-authored-by: author";
pub const WHITESPACE: &str = " ";
pub const COMMENT: &str = "#comment!";

pub struct TestMessage {
	pub subject: String,
	pub body: String,
	pub authors: Vec<String>,
}

pub struct TestCase {
	pub name: &'static str,
	pub message: TestMessage,
	pub expected: String,
}

impl TestCase {
	pub fn build_for(name: &'static str) -> TestCaseBuilder {
		TestCaseBuilder::new(name)
	}
}

pub struct TestCaseBuilder {
	test_case: TestCase,
}

impl TestCaseBuilder {
	pub fn new(name: &'static str) -> Self {
		TestCaseBuilder {
			test_case: TestCase {
				name,
				message: TestMessage {
					subject: "UNDEFINED".to_string(),
					body: "UNDEFINED".to_string(),
					authors: vec!["UNDEFINED".to_string()],
				},
				expected: "UNDEFINED".to_string(),
			},
		}
	}

	pub fn subject(mut self, subject: &str) -> Self {
		self.test_case.message.subject = subject.to_string();
		self
	}

	pub fn body(mut self, body: &str) -> Self {
		self.test_case.message.body = body.to_string();
		self
	}

	pub fn authors(mut self, authors: &[&str]) -> Self {
		self.test_case.message.authors = authors.iter().map(ToString::to_string).collect();
		self
	}

	pub fn expected(mut self, expected: &str) -> Self {
		self.test_case.expected = expected.to_string();
		self
	}

	pub fn create(self) -> TestCase {
		self.test_case
	}
}
