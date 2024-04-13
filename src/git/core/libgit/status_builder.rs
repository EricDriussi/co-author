use crate::{git::err::GitError, Result};
use git2::{Repository, StatusEntry, StatusOptions, Statuses};

pub fn for_editmsg(repo: &Repository) -> Result<String> {
	let mut options = StatusOptions::new();
	options.include_untracked(true);

	let head = repo.head()?;
	let branch_name = head
		.shorthand()
		.ok_or_else(|| GitError::LibGit("Could not get branch name".to_string()))?;
	let file_statuses = repo.statuses(Some(&mut options))?;

	let heading = format!(
		"

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# A message with only 'Co-Authored' lines will be considered empty.
#
# On branch {branch_name}\n"
	);

	Ok(format!(
		"{}{}{}{}",
		heading,
		changes_to_be_committed(&file_statuses),
		changes_not_staged_for_commit(&file_statuses),
		untracked_files(&file_statuses)
	))
}

fn changes_to_be_committed(file_statuses: &Statuses) -> String {
	let heading = "# Changes to be committed:";
	let content = file_statuses
		.iter()
		.filter(|file| {
			file.status().is_index_new()
				|| file.status().is_index_modified()
				|| file.status().is_index_deleted()
				|| file.status().is_index_renamed()
				|| file.status().is_index_typechange()
		})
		.filter_map(format_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{heading}\n{content}")
	}
}

fn changes_not_staged_for_commit(file_statuses: &Statuses) -> String {
	let heading = "#\n# Changes not staged for commit:";
	let content = file_statuses
		.iter()
		.filter(|file| file.status().is_wt_modified())
		.filter_map(format_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{heading}\n{content}")
	}
}

fn untracked_files(file_statuses: &Statuses) -> String {
	let heading = "#\n# Untracked files:";
	let content = file_statuses
		.iter()
		.filter(|file| file.status().is_wt_new())
		.filter_map(format_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{heading}\n{content}")
	}
}

#[allow(clippy::needless_pass_by_value)]
fn format_path(file: StatusEntry) -> Option<String> {
	file.path().map(|path| format!("#\t{path}\n"))
}
