pub enum CommitMode<'a> {
	WithEditor {
		message: Option<&'a str>,
		authors: Vec<String>,
		amend: bool,
	},
	WithoutEditor {
		message: &'a str,
		authors: Vec<String>,
		amend: bool,
	},
}
