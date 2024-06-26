use crate::{
	args::Args,
	authors::author::{Author, AuthorsProvider},
	git::{commit_mode::CommitMode, di::Service},
	ui::cli::Cli,
	Result,
};

pub struct Orchestrator {
	args: Args,
	cli: Cli,
	service: Service,
	provider: Box<dyn AuthorsProvider>,
}

impl Orchestrator {
	pub fn exec(args: Args, cli: Cli, service: Service, provider: Box<dyn AuthorsProvider>) -> Result<()> {
		let mut orch = Self {
			args,
			cli,
			service,
			provider,
		};
		let authors_signatures = orch.get_authors()?;
		orch.commit(authors_signatures)
	}

	fn get_authors(&mut self) -> Result<Vec<String>> {
		let all_authors = self.provider.all();
		if self.args.all {
			let all_signatures: Vec<_> = all_authors.iter().map(Author::signature).collect();
			return if self.args.sort {
				Ok(Self::sort(all_signatures))
			} else {
				Ok(all_signatures)
			};
		}

		if self.args.fzf {
			let found_authors: Vec<_> = self
				.provider
				.find_by_hashes(&self.cli.fzf_prompt(&all_authors)?)
				.iter()
				.map(Author::signature)
				.collect();
			return if self.args.sort {
				Ok(Self::sort(found_authors))
			} else {
				Ok(found_authors)
			};
		}

		let aliases = match &self.args.list {
			Some(list) => list.split(',').map(ToString::to_string).collect::<Vec<String>>(),
			None => self.cli.aliases_prompt(&all_authors)?,
		};
		let found_authors: Vec<_> = self
			.provider
			.find_by_aliases(&aliases)
			.iter()
			.map(Author::signature)
			.collect();

		if self.args.sort {
			Ok(Self::sort(found_authors))
		} else {
			Ok(found_authors)
		}
	}

	fn commit(&mut self, authors_signatures: Vec<String>) -> Result<()> {
		if self.args.amend {
			self.args.pre_populate = true;
		}

		if self.args.editor {
			if self.args.pre_populate {
				return self.service.commit(CommitMode::WithEditor {
					message: Some(self.service.last_commit_message().as_str()),
					authors: authors_signatures,
					amend: self.args.amend,
				});
			}
			return self.service.commit(CommitMode::WithEditor {
				message: None,
				authors: authors_signatures,
				amend: self.args.amend,
			});
		}

		let msg = match (self.args.message.clone(), self.args.pre_populate) {
			(Some(msg), _) => msg,
			(None, false) => self.cli.message_prompt()?,
			(None, true) => self
				.cli
				.pre_populated_message_prompt(&self.service.last_commit_message())?,
		};

		self.service.commit(CommitMode::WithoutEditor {
			message: msg.as_str(),
			authors: authors_signatures,
			amend: self.args.amend,
		})
	}

	fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
		vector.sort();
		vector
	}
}
