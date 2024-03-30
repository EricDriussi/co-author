pub mod commit_mode;
pub mod di;
pub mod err;
mod service;

mod core {
	pub mod commit_message;
	pub mod conf_provider;
	pub mod hook;

	pub mod libgit {
		mod status_formatter;
		pub mod wrapper;
		#[cfg(test)]
		mod wrapper_should;
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
			mod commit_with_editor_should;
			mod commit_without_editor_should;
			mod mock_helpers;
			mod service_should;
		}
	}
}
