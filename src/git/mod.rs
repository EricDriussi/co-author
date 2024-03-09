mod commit_message;
pub mod commit_mode;
mod conf_provider;
pub mod di;
mod editor;
mod err;
mod hook;
mod libgit_wrapper;
mod service;

#[cfg(test)]
mod test {
	mod editor_should;
	mod git_should;
	mod hook_should;
	mod libgit_wrapper_should;
	mod service {
		mod commit_with_editor_should;
		mod commit_without_editor_should;
		mod service_should;

		pub mod util {
			pub mod mock_file;
			pub mod mock_helpers;
		}
	}
}
