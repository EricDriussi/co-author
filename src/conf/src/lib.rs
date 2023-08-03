use std::env;

use config::Config;

const DEFAULT_CONFIG: &'static str = include_str!("configs/default.yaml");
const TEST_CONFIG: &'static str = include_str!("configs/test.yaml");

pub fn authors_file() -> String {
	return get_config().get::<String>("authors_file").unwrap();
}

fn get_config() -> Config {
	let config_file = if let Ok(test_env) = env::var("COA_ENV") {
		if test_env == "test" {
			TEST_CONFIG
		} else {
			DEFAULT_CONFIG
		}
	} else {
		DEFAULT_CONFIG
	};

	return Config::builder()
		.add_source(config::File::from_str(config_file, config::FileFormat::Yaml))
		.build()
		.unwrap();
}
