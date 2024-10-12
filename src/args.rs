use clap::Parser;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// CSV file containing a list of authors (alias,name,email)
	#[arg(short, long)]
	pub file: Option<String>,

	/// List of comma separated author aliases
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

	/// Sort authors signatures when adding to commit message
	#[arg(short, long, default_value = "false")]
	pub sort: bool,

	/// Amend last commit, both message and authors will be overwritten
	#[arg(long, default_value = "false")]
	pub amend: bool,

	/// Use fzf for author selection
	#[arg(long, default_value = "false", conflicts_with("list"), conflicts_with("all"))]
	pub fzf: bool,
	// FIXME: if multiple authors have the same alias, selecting one with fzf will add them all as co-authors
	// TODO: instead of integrated fzf functionality, eval if atty crate is a better fit
}
