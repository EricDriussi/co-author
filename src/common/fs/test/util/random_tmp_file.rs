use std::fs;
use uuid::Uuid;

const DIR_PATH: &str = "/tmp/coa/files";

pub fn create() -> (fs::File, String) {
	let random = Uuid::new_v4();
	let file_path = format!("{DIR_PATH}/{random}");

	fs::create_dir_all(DIR_PATH).expect("Could not create random file for test");
	(
		fs::File::create(&file_path).expect("Could not create random file for test"),
		file_path,
	)
}

pub fn path() -> String {
	let random = Uuid::new_v4();
	format!("{DIR_PATH}/{random}")
}
