use git2::Config;

#[cfg_attr(test, mockall::automock)]
pub trait DefaultEditorGetter {
	fn get_editor(&self) -> Option<String>;
}

pub struct GitConfProvider;

impl GitConfProvider {
	pub fn new() -> Self {
		Self {}
	}
}

impl DefaultEditorGetter for GitConfProvider {
	fn get_editor(&self) -> Option<String> {
		Config::open_default().ok()?.get_string("core.editor").ok()
	}
}
