use std::env;

use config::{Config, Environment, File, FileFormat};

const DEFAULT_CONFIG: &str = include_str!("configs/default.yaml");
const TEST_CONFIG: &str = include_str!("configs/test.yaml");
const CONFIG_ERR_MSG: &str = "Config not loaded properly";

pub fn authors_csv_path() -> String {
	let file_name = authors_csv_file();
	let path_to_config_dir = get_config().get::<String>("authors_path").expect(CONFIG_ERR_MSG);
	let full_file_path = format!("{path_to_config_dir}{file_name}");
	let home_placeholder = "PLACEHOLDER";

	if let Ok(env_home_var) = env::var("XDG_CONFIG_HOME") {
		full_file_path.replace(&format!("${home_placeholder}"), &env_home_var)
	} else {
		let env_home_var = env::var("HOME").expect("Your $HOME is not set, can't locate default authors file path!");
		full_file_path.replace(&format!("${home_placeholder}"), &format!("{env_home_var}/.config"))
	}
}

pub fn authors_csv_file() -> String {
	get_config().get::<String>("authors_csv_name").expect(CONFIG_ERR_MSG)
}

pub fn dummy_data() -> String {
	get_config().get::<String>("dummy_data").expect(CONFIG_ERR_MSG)
}

pub fn hooks_path() -> String {
	get_config().get::<String>("hooks_path").expect(CONFIG_ERR_MSG)
}

pub fn editmsg() -> String {
	get_config().get::<String>("editmsg").expect(CONFIG_ERR_MSG)
}

fn get_config() -> Config {
	let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "default".to_string());

	match run_mode.as_str() {
		"test" => Config::builder()
			.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
			// optional config, overrides default
			.add_source(File::from_str(TEST_CONFIG, FileFormat::Yaml))
			// allow settings from the environment (with a prefix of APP)
			.add_source(Environment::with_prefix("app"))
			.build()
			.expect(CONFIG_ERR_MSG),

		_ => Config::builder()
			.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
			// allow settings from the environment (with a prefix of APP)
			.add_source(Environment::with_prefix("app"))
			.build()
			.expect(CONFIG_ERR_MSG),
	}
}
