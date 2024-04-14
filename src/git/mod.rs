pub mod commit_mode;
pub mod di;
pub mod err;

mod core {
	mod commit_message;
	pub mod conf_provider;
	pub mod hook;
	pub mod service;

	pub mod libgit {
		mod status_builder;
		pub mod wrapper;

		#[cfg(test)]
		mod test {
			mod helper;
			mod wrapper_should;
			mod wrapper_should_amend;
			mod wrapper_should_build;
			mod wrapper_should_commit;
		}
	}

	pub mod editor {
		pub mod file_editor;
		#[cfg(test)]
		mod file_editor_should;
	}

	#[cfg(test)]
	mod test {
		mod commit_message {
			mod helper;
			mod should_create;
			mod should_parse;
		}

		mod hook_should;
		mod service {
			mod amend_with_editor_should;
			mod amend_without_editor_should;
			mod commit_with_editor_should;
			mod commit_without_editor_should;
			mod mock_helpers;
			mod service_should;
		}
	}
}
