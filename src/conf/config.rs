use std::env;

use config::Config;

const DEFAULT_CONFIG: &str = include_str!("configs/default.yaml");
const TEST_CONFIG: &str = include_str!("configs/test.yaml");

pub fn authors_file_path() -> String {
	let path = get_config().get::<String>("authors_file_path").unwrap();
	let file = authors_file_name();
	let raw_config_string = format!("{}{}", path, file);
	let base = "BASE_PATH";
	match env::var("XDG_CONFIG_HOME") {
		Ok(env_var) => raw_config_string.replace(&format!("${}", base), &env_var),
		Err(_) => match env::var("HOME") {
			Ok(env_var) => raw_config_string.replace(&format!("${}", base), &format!("{}/.config", env_var)),
			Err(_) => panic!("Your $HOME is not set, can't locate authors file!"),
		},
	}
}

pub fn authors_file_name() -> String {
	get_config().get::<String>("authors_file_name").unwrap()
}

pub fn dummy_data() -> String {
	get_config().get::<String>("dummy_data").unwrap()
}

pub fn hooks_path() -> String {
	get_config().get::<String>("hooks_path").unwrap()
}

pub fn editmsg() -> String {
	get_config().get::<String>("editmsg").unwrap()
}

pub fn get_config() -> Config {
	let config_file = if let Ok(test_env) = env::var("COA_ENV") {
		if test_env == "test" {
			TEST_CONFIG
		} else {
			DEFAULT_CONFIG
		}
	} else {
		DEFAULT_CONFIG
	};

	Config::builder()
		.add_source(config::File::from_str(config_file, config::FileFormat::Yaml))
		.build()
		.unwrap()
}
