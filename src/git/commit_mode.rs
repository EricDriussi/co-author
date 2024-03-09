pub enum CommitMode<'a> {
	WithEditor {
		message: Option<&'a str>,
		authors: Vec<String>,
	},
	WithoutEditor {
		message: &'a str,
		authors: Vec<String>,
	},
}
