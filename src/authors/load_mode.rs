use crate::common::fs::wrapper::FileLoader;

pub enum LoadMode<'a> {
	FromCwd {
		file_loader: &'a dyn FileLoader,
	},
	FromPath {
		file_loader: &'a dyn FileLoader,
		path: &'a str,
	},
}
