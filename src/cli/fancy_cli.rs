use super::input_reader::InputReader;
use crate::authors::author::Author;
use crate::Result;
use colored::Colorize;

pub struct FancyCli {
	prompt: Box<dyn InputReader>,
}

impl FancyCli {
	pub fn new(editor: impl InputReader + 'static) -> Self {
		Self {
			prompt: Box::new(editor),
		}
	}

	pub fn prompt_commit_message(&mut self) -> Result<String> {
		Ok(self.prompt.readline("Enter your commit message:\n")?.trim().to_string())
	}

	pub fn prompt_aliases(&mut self, authors: &[Author]) -> Result<Vec<String>> {
		let formatted_authors = Self::prettify_authors(authors);
		let aliases = self.prompt.readline(&format!(
			"\n{formatted_authors}\n\nEnter co-authors aliases separated by spaces:\n"
		))?;
		Ok(aliases.split_whitespace().map(ToString::to_string).collect())
	}

	pub fn prompt_pre_populated_commit_message(&mut self, prev_commit_msg: &str) -> Result<String> {
		Ok(self
			.prompt
			.readline_with_initial("Enter your commit message:\n", (prev_commit_msg, ""))?
			.trim()
			.to_string())
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
