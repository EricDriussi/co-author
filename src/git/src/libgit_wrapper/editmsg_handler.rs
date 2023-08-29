use git2::{Repository, StatusEntry, StatusOptions, Statuses};

pub fn get_status_for_commit_file(repo: &Repository) -> String {
	let mut options = StatusOptions::new();
	options.include_untracked(true);

	let head = repo.head().unwrap();
	let short_head = head.shorthand().unwrap();
	let file_statuses = repo.statuses(Some(&mut options)).unwrap();

	let output = format!("\n# On branch {}\n", short_head);

	format!(
		"{}# Changes to be committed:\n{}#\n# Changes not staged for commit:\n{}#\n# Untracked files:\n{}",
		output,
		changes_to_be_committed(&file_statuses),
		changes_not_staged_for_commit(&file_statuses),
		untracked_files(&file_statuses)
	)
}

fn changes_to_be_committed(file_statuses: &Statuses) -> String {
	file_statuses
		.iter()
		.filter(|file| {
			file.status().is_index_new()
				|| file.status().is_index_modified()
				|| file.status().is_index_deleted()
				|| file.status().is_index_renamed()
				|| file.status().is_index_typechange()
		})
		.map(format_file_path)
		.collect::<String>()
}

fn changes_not_staged_for_commit(file_statuses: &Statuses) -> String {
	file_statuses
		.iter()
		.filter(|file| file.status().is_wt_modified())
		.map(format_file_path)
		.collect::<String>()
}

fn untracked_files(file_statuses: &Statuses) -> String {
	file_statuses
		.iter()
		.filter(|file| file.status().is_wt_new())
		.map(format_file_path)
		.collect::<String>()
}

fn format_file_path(entry: StatusEntry) -> String {
	return format!("#\t{}\n", entry.path().unwrap());
}
