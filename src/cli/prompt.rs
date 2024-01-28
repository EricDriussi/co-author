use super::input_reader::Reader;
use crate::authors::author::Author;
use crate::Result;
use colored::Colorize;

pub struct Prompt<T: Reader> {
	reader: T,
}

impl<T: Reader> Prompt<T> {
	pub fn new(reader: T) -> Self {
		Self { reader }
	}

	pub fn prompt_commit_message(&mut self) -> Result<String> {
		let prompt_msg = "Enter your commit message:";
		let input = self.reader.readline(&format!("{prompt_msg}\n"))?;
		Ok(input.trim().to_string())
	}

	pub fn prompt_aliases(&mut self, authors: &[Author]) -> Result<Vec<String>> {
		let pretty_authors = Self::prettify_authors(authors);
		let prompt_msg = "Enter co-authors aliases separated by spaces:";
		let input = self.reader.readline(&format!("\n{pretty_authors}\n\n{prompt_msg}\n"))?;
		Ok(input.split_whitespace().map(ToString::to_string).collect())
	}

	pub fn prompt_pre_populated_commit_message(&mut self, prev_commit_msg: &str) -> Result<String> {
		let prompt_msg = "Update your commit message:";
		let input = self
			.reader
			.readline_with_initial(&format!("{prompt_msg}\n"), (prev_commit_msg, ""))?;
		Ok(input.trim().to_string())
	}

	fn prettify_authors(authors: &[Author]) -> String {
		authors.iter().map(Self::prettify).collect::<Vec<String>>().join("\n")
	}

	fn prettify(author: &Author) -> String {
		format!(
			"{} {} {} {}",
			"⦔".yellow(),
			author.alias().blue(),
			"->".green(),
			author.name()
		)
	}
}
