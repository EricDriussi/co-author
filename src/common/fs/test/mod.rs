use std::fs;

use uuid::Uuid;

#[cfg(test)]
mod file;
#[cfg(test)]
mod wrapper;

pub fn create_random_tmp_file() -> (fs::File, String) {
	let random = Uuid::new_v4();
	let dir_path = format!("/tmp/coa");
	let file_path = format!("{}/{}", dir_path, random);

	fs::create_dir_all(&dir_path).expect("Could not create random file for test");
	(
		fs::File::create(&file_path).expect("Could not create random file for test"),
		file_path,
	)
}
