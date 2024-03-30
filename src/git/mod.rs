mod commit_message;
pub mod commit_mode;
pub mod di;
pub mod err;
mod hook;
mod service;

mod libgit {
	pub mod editmsg_status_formatter;
	pub mod wrapper;
	#[cfg(test)]
	mod wrapper_should;
}

mod editor {
	pub mod conf_provider;
	pub mod simple_editor;

	#[cfg(test)]
	mod simple_editor_should;
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
		mod service_should;

		pub mod util {
			pub mod mock_helpers;
		}
	}
}
