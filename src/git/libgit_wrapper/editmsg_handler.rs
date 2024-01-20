use std::{
	error::Error,
	io::{BufRead, BufReader},
};

use co_author::conf;
use git2::{Repository, StatusEntry, StatusOptions, Statuses};

use crate::git::commit_body::CommitBody;

const ERR_MSG: &str = "GIT ERROR";

pub fn write_commit_to_file(commit_body: &CommitBody) -> Result<(), Box<dyn Error>> {
	std::fs::write(conf::editmsg(), commit_body.formatted_body())?;
	Ok(())
}

pub fn read_editmsg() -> Option<String> {
	read(conf::editmsg())
}

fn read(editmsg_path: String) -> Option<String> {
	let file = std::fs::File::open(editmsg_path).expect("Something went wrong");
	let reader = BufReader::new(file);
	let mut commit_body = String::new();

	for line in reader.lines().flatten() {
		if !line.starts_with('#') {
			commit_body.push_str(line.trim());
			commit_body.push('\n');
		}
	}
	let trimmed_body = commit_body.trim().to_string();

	if has_message(&trimmed_body) {
		Some(trimmed_body)
	} else {
		None
	}
}

fn has_message(commit_body: &str) -> bool {
	let lines_without_co_author = commit_body
		.lines()
		.filter(|line| !line.starts_with("Co-Authored-by"))
		.collect::<Vec<&str>>()
		.join("\n");

	let contains_lines_without_co_author = !lines_without_co_author.trim().is_empty();
	contains_lines_without_co_author
}

pub fn get_status_for_commit_file(repo: &Repository) -> String {
	let mut options = StatusOptions::new();
	options.include_untracked(true);

	let head = repo.head().expect(ERR_MSG);
	let branch_name = head.shorthand().expect(ERR_MSG);
	let file_statuses = repo.statuses(Some(&mut options)).expect(ERR_MSG);

	let heading = format!(
		"

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# A message with only 'Co-Authored' lines will be considered empty.
#
# On branch {branch_name}\n"
	);

	format!(
		"{}{}{}{}",
		heading,
		changes_to_be_committed(&file_statuses),
		changes_not_staged_for_commit(&file_statuses),
		untracked_files(&file_statuses)
	)
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

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_removes_commented_lines_when_reading_commit_message() {
		let commit_editmsg_path = ".git/COMMIT_EDITMSG_TEST_COMMENTS";
		std::fs::write(
			commit_editmsg_path,
			"Test commit message.\n# This is a commented line.\n#And another one."
				.to_string()
				.clone(),
		)
		.unwrap();

		let result = read(commit_editmsg_path.to_string());

		assert_eq!(result, Some("Test commit message.".to_string()));

		// Cleanup
		std::fs::remove_file(commit_editmsg_path).unwrap();
	}

	#[test]
	fn test_trims_lines_when_reading_commit_message() {
		let test_commit_message = "  Test commit message.\nThis is a second line. \n".to_string();
		let commit_editmsg_path = ".git/COMMIT_EDITMSG_TEST_TRIM";
		std::fs::write(commit_editmsg_path, test_commit_message.clone()).unwrap();

		let result = read(commit_editmsg_path.to_string());

		assert_eq!(result, Some(test_commit_message.trim().to_string()));

		// Cleanup
		std::fs::remove_file(commit_editmsg_path).unwrap();
	}
}
