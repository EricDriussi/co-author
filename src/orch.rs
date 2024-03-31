use crate::{
	args::Args,
	authors::{
		self,
		author::{Author, AuthorsProvider},
	},
	git::{self, commit_mode::CommitMode, di::Service},
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
	pub fn new(args: Args, cli: Cli) -> Result<Self> {
		let git_service = git::di::init()?;
		let authors_prov = match &args.file {
			Some(file) => authors::di::init_for(file)?,
			None => authors::di::init()?,
		};
		Ok(Self {
			args,
			cli,
			service: git_service,
			provider: authors_prov,
		})
	}

	pub fn handle_authors(&mut self) -> Result<Vec<String>> {
		if self.args.all {
			return if self.args.sort {
				Ok(Self::sort(self.provider.all().iter().map(Author::signature).collect()))
			} else {
				Ok(self.provider.all().iter().map(Author::signature).collect())
			};
		}
		if let Some(list) = &self.args.list {
			let given_aliases = list.split(',').map(ToString::to_string).collect::<Vec<String>>();
			return if self.args.sort {
				Ok(Self::sort(
					self.provider
						.find(&given_aliases)
						.iter()
						.map(Author::signature)
						.collect(),
				))
			} else {
				Ok(self
					.provider
					.find(&given_aliases)
					.iter()
					.map(Author::signature)
					.collect())
			};
		}

		let aliases = self.cli.prompt_for_aliases(&self.provider.all())?;
		if self.args.sort {
			Ok(Self::sort(
				self.provider.find(&aliases).iter().map(Author::signature).collect(),
			))
		} else {
			Ok(self.provider.find(&aliases).iter().map(Author::signature).collect())
		}
	}

	pub fn handle_commit_msg(&mut self, authors_signatures: Vec<String>) -> Result<()> {
		let prev = self.service.last_commit_message();

		if self.args.editor {
			if self.args.pre_populate {
				return self
					.service
					.commit(CommitMode::WithEditor {
						message: Some(prev.as_str()),
						authors: authors_signatures,
					})
					.map_err(Into::into);
			}
			return self
				.service
				.commit(CommitMode::WithEditor {
					message: None,
					authors: authors_signatures,
				})
				.map_err(Into::into);
		}

		let msg = match (self.args.message.clone(), self.args.pre_populate) {
			(Some(msg), _) => msg,
			(None, false) => self.cli.prompt_for_message()?,
			(None, true) => self.cli.prompt_for_pre_populated_message(&prev)?,
		};

		self.service
			.commit(CommitMode::WithoutEditor {
				message: msg.as_str(),
				authors: authors_signatures,
			})
			.map_err(Into::into)
	}

	fn sort<String: Ord>(mut vector: Vec<String>) -> Vec<String> {
		vector.sort();
		vector
	}
}
