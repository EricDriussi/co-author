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
		if self.args.all {
			let all_authors: Vec<_> = self.provider.all().iter().map(Author::signature).collect();
			return if self.args.sort {
				Ok(Self::sort(all_authors))
			} else {
				Ok(all_authors)
			};
		}

		let aliases = match &self.args.list {
			Some(list) => list.split(',').map(ToString::to_string).collect::<Vec<String>>(),
			None => self.cli.aliases_prompt(&self.provider.all())?,
		};
		let found_authors: Vec<_> = self.provider.find(&aliases).iter().map(Author::signature).collect();

		if self.args.sort {
			Ok(Self::sort(found_authors))
		} else {
			Ok(found_authors)
		}
	}

	fn commit(&mut self, authors_signatures: Vec<String>) -> Result<()> {
		if self.args.editor {
			if self.args.pre_populate {
				return self.service.commit(CommitMode::WithEditor {
					message: Some(self.service.last_commit_message().as_str()),
					authors: authors_signatures,
				});
			}
			return self.service.commit(CommitMode::WithEditor {
				message: None,
				authors: authors_signatures,
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
		})
	}

	fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
		vector.sort();
		vector
	}
}
