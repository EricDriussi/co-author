use std::{
	io::{BufRead, BufReader},
	path::{Path, PathBuf},
};

use git2::{Repository, StatusEntry, StatusOptions, Statuses};

use crate::git_domain::CommitBody;

pub fn write_commit_to_file(commit_body: CommitBody, editmsg_path: PathBuf) -> Result<(), String> {
	return match std::fs::write(editmsg_path, commit_body.formatted_body()) {
		Ok(_) => Ok(()),
		Err(_) => Err("Something went wrong".to_string()),
	};
}

pub fn read_editmsg(editmsg_path: &Path) -> Option<String> {
	let file = std::fs::File::open(editmsg_path).expect("Something went wrong");
	let reader = BufReader::new(file);
	let mut commit_body = String::new();

	for line in reader.lines() {
		if let Ok(line) = line {
			if !line.starts_with('#') {
				commit_body.push_str(&line.trim());
				commit_body.push('\n');
			}
		}
	}
	let trimmed_body = commit_body.trim().to_string();

	match has_message(&trimmed_body) {
		true => Some(trimmed_body),
		false => None,
	}
}

fn has_message(commit_body: &String) -> bool {
	let lines_without_co_author = commit_body
		.lines()
		.filter(|line| !line.starts_with("Co-Authored-by"))
		.collect::<Vec<&str>>()
		.join("\n");

	let contains_lines_without_co_author = !lines_without_co_author.trim().is_empty();
	return contains_lines_without_co_author;
}

pub fn get_status_for_commit_file(repo: &Repository) -> String {
	let mut options = StatusOptions::new();
	options.include_untracked(true);

	let head = repo.head().unwrap();
	let branch_name = head.shorthand().unwrap();
	let file_statuses = repo.statuses(Some(&mut options)).unwrap();

	let heading = format!(
		"

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# A message with only 'Co-Authored' lines will be considered empty.
#
# On branch {}\n",
		branch_name
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
		.map(format_file_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{}\n{}", heading.to_string(), content)
	}
}

fn changes_not_staged_for_commit(file_statuses: &Statuses) -> String {
	let heading = "#\n# Changes not staged for commit:";
	let content = file_statuses
		.iter()
		.filter(|file| file.status().is_wt_modified())
		.map(format_file_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{}\n{}", heading.to_string(), content)
	}
}

fn untracked_files(file_statuses: &Statuses) -> String {
	let heading = "#\n# Untracked files:";
	let content = file_statuses
		.iter()
		.filter(|file| file.status().is_wt_new())
		.map(format_file_path)
		.collect::<String>();

	if content.is_empty() {
		String::new()
	} else {
		format!("{}\n{}", heading.to_string(), content)
	}
}

fn format_file_path(entry: StatusEntry) -> String {
	return format!("#\t{}\n", entry.path().unwrap());
}

#[cfg(test)]
mod test {

	use std::path::Path;

	use super::*;

	#[test]
	fn test_removes_commented_lines_when_reading_commit_message() {
		let commit_editmsg_path = "../../.git/COMMIT_EDITMSG_TEST_COMMENTS";
		std::fs::write(
			commit_editmsg_path.clone(),
			"Test commit message.\n# This is a commented line.\n#And another one."
				.to_string()
				.clone(),
		)
		.unwrap();

		let result = read_editmsg(&Path::new(commit_editmsg_path));

		assert_eq!(result, Some("Test commit message.".to_string()));

		// Cleanup
		std::fs::remove_file(commit_editmsg_path).unwrap();
	}

	#[test]
	fn test_trims_lines_when_reading_commit_message() {
		let test_commit_message = "  Test commit message.\nThis is a second line. \n".to_string();
		let commit_editmsg_path = "../../.git/COMMIT_EDITMSG_TEST_TRIM";
		std::fs::write(commit_editmsg_path.clone(), test_commit_message.clone()).unwrap();

		let result = read_editmsg(&Path::new(commit_editmsg_path));

		assert_eq!(result, Some(test_commit_message.trim().to_string()));

		// Cleanup
		std::fs::remove_file(commit_editmsg_path).unwrap();
	}
}
