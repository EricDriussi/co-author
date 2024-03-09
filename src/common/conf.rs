use config::{Config, Environment, File, FileFormat};

const CONFIG: &str = include_str!("./../../config.yaml");
const CONFIG_ERR_MSG: &str = "Config not loaded properly";

pub fn authors_dir() -> String {
	get_config().get::<String>("authors_dir").expect(CONFIG_ERR_MSG)
}

pub fn authors_file() -> String {
	get_config().get::<String>("authors_file").expect(CONFIG_ERR_MSG)
}

pub fn hooks_path() -> String {
	get_config().get::<String>("hooks_path").expect(CONFIG_ERR_MSG)
}

pub fn editmsg() -> String {
	get_config().get::<String>("editmsg").expect(CONFIG_ERR_MSG)
}

pub fn co_author_prefix() -> String {
	get_config().get::<String>("co_author_prefix").expect(CONFIG_ERR_MSG)
}

fn get_config() -> Config {
	Config::builder()
		.add_source(File::from_str(CONFIG, FileFormat::Yaml))
		// allow settings from the environment (with a prefix of APP)
		.add_source(Environment::with_prefix("app"))
		.build()
		.expect(CONFIG_ERR_MSG)
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn should_return_expected_test_authors_file_path() {
		let authors_dir = authors_dir();
		assert_eq!(authors_dir, "co-author");
	}

	#[test]
	fn should_return_expected_test_authors_file_name() {
		let authors_file = authors_file();
		assert_eq!(authors_file, "authors.csv");
	}

	#[test]
	fn should_return_expected_test_hooks_path() {
		let hooks_path = hooks_path();
		assert_eq!(hooks_path, ".git/hooks");
	}

	#[test]
	fn should_return_expected_test_editmsg_file() {
		let commit_editmsg = editmsg();
		assert_eq!(commit_editmsg, ".git/COMMIT_EDITMSG");
	}
}
