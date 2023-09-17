use clap::Parser;

/// Co-author your git commits
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// File containing a csv formatted list of authors (alias,name,email)
	#[arg(short, long)]
	pub file: Option<String>,

	/// List of comma spearated author aliases
	#[arg(short, long)]
	pub list: Option<String>,

	/// Use all available authors
	#[arg(short, long, conflicts_with("list"), default_value = "false")]
	pub all: bool,

	/// Specify commit message
	#[arg(short, long, conflicts_with("editor"), conflicts_with("pre_populate"))]
	pub message: Option<String>,

	/// Open default editor for commit message
	#[arg(short, long, default_value = "false")]
	pub editor: bool,

	/// Pre-populate prompt/editor with (first line of) last commit message
	#[arg(short, long, default_value = "false")]
	pub pre_populate: bool,
}
