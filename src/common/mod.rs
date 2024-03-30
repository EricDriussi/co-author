pub mod conf;
pub mod runner;

pub mod fs {
	pub mod file;
	pub mod file_reader;
	pub mod file_writer;

	#[cfg(test)]
	pub mod test {
		mod file_reader_should;
		mod file_should;
		mod file_writer_should;

		pub mod util {
			pub mod random_tmp_file;
		}
	}
}
