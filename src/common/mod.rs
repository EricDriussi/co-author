pub mod conf;
pub mod file_reader;
pub mod runner;

pub mod fs {
	pub mod file;
	pub mod wrapper;

	#[cfg(test)]
	pub mod test {
		mod file_should;
		mod wrapper_should;

		pub mod util {
			pub mod dummy_file;
			pub mod random_tmp_file;
		}
	}
}
