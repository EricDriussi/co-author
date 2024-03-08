use std::fs;
use uuid::Uuid;

pub fn create() -> (fs::File, String) {
	let random = Uuid::new_v4();
	let dir_path = "/tmp/coa/files";
	let file_path = format!("{dir_path}/{random}");

	fs::create_dir_all(dir_path).expect("Could not create random file for test");
	(
		fs::File::create(&file_path).expect("Could not create random file for test"),
		file_path,
	)
}
