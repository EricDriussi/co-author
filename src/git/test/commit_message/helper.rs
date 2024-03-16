pub const EMPTY: &str = "";
pub const A_SUBJECT: &str = "subject line";
pub const A_LINE: &str = "line";
pub const AN_AUTHOR: &str = "Co-authored-by: author";
pub const WHITESPACE: &str = " ";
pub const COMMENT: &str = "#comment!";
pub const NO_AUTHORS: Vec<String> = vec![];

pub struct TestCase {
	pub name: &'static str,
	pub subject: String,
	pub body: String,
	pub authors: Vec<String>,
	pub expected: String,
}
