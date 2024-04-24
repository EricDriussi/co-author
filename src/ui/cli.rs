use super::err::UiError;
use super::input_reader::InputReader;
use crate::authors::author::Author;
use crate::common::runner::Runner;
use crate::Result;
use colored::Colorize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

const FZF_SEPARATOR: &str = " - ";

pub struct Cli {
	reader: Box<dyn InputReader>,
	runner: Box<dyn Runner>,
}

impl Cli {
	pub fn new(reader: Box<dyn InputReader>, runner: Box<dyn Runner>) -> Self {
		Self { reader, runner }
	}

	pub fn message_prompt(&mut self) -> Result<String> {
		let prompt_msg = "Enter commit message:";
		let input = self.reader.readline(&format!("{prompt_msg}\n"))?;
		Ok(input.trim().to_string())
	}

	pub fn aliases_prompt(&mut self, authors: &[Author]) -> Result<Vec<String>> {
		let pretty_authors = Self::prettify_authors(authors);
		let prompt_msg = "Enter co-authors aliases separated by spaces:";
		let input = self.reader.readline(&format!("\n{pretty_authors}\n\n{prompt_msg}\n"))?;
		Ok(input.split_whitespace().map(ToString::to_string).collect())
	}

	pub fn pre_populated_message_prompt(&mut self, prev_commit_msg: &str) -> Result<String> {
		let prompt_msg = "Update commit message:";
		let input = self
			.reader
			.readline_with_prompt(&format!("{prompt_msg}\n"), (prev_commit_msg, ""))?;
		Ok(input.trim().to_string())
	}

	pub fn fzf_prompt(&self, authors: &[Author]) -> Result<Vec<u64>> {
		let mut fzf_proc = self
			.runner
			.attach("fzf", &["--multi".to_string(), "--ansi".to_string()])?;
		let stdin = fzf_proc
			.stdin
			.as_mut()
			.ok_or(UiError::Fzf("Could not attach stdin".to_string()))?;

		for author in authors.iter().map(Self::fzf_format) {
			writeln!(stdin, "{author}").map_err(|_| UiError::Fzf("Could not pipe to stdin".to_string()))?;
		}

		let output = fzf_proc
			.wait_with_output()
			.map_err(|_| UiError::Fzf("Could not read output".to_string()))?;
		let selected_aliases: Vec<u64> = String::from_utf8_lossy(&output.stdout)
			.lines()
			.map(|line| Self::hash_of(&line.replace(FZF_SEPARATOR, "")))
			.collect();

		Ok(selected_aliases)
	}

	fn hash_of(str: &str) -> u64 {
		let mut hasher = DefaultHasher::new();
		str.hash(&mut hasher);
		hasher.finish()
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

	fn fzf_format(author: &Author) -> String {
		format!("{}{}{}", author.alias().blue(), FZF_SEPARATOR, author.name())
	}
}
