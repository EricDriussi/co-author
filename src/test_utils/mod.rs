pub mod dummy_file;
pub mod file_cleanup;
pub fn set_test_env() {
	std::env::set_var("RUN_MODE", "test");
}
