use std::env;

use config::{Config, Environment, File, FileFormat};

const DEFAULT_CONFIG: &str = include_str!("configs/default.yaml");
const TEST_CONFIG: &str = include_str!("configs/test.yaml");

pub fn authors_file_path() -> String {
	let path_to_config_dir = get_config().get::<String>("authors_file_path").unwrap();
	let file_name = authors_file_name();
	let full_file_path = format!("{}{}", path_to_config_dir, file_name);
	let home_placeholder = "PLACEHOLDER";
	match env::var("XDG_CONFIG_HOME") {
		Ok(env_home_var) => full_file_path.replace(&format!("${}", home_placeholder), &env_home_var),
		Err(_) => match env::var("HOME") {
			Ok(env_home_var) => {
				full_file_path.replace(&format!("${}", home_placeholder), &format!("{}/.config", env_home_var))
			}
			Err(_) => panic!("Your $HOME is not set, can't locate default authors file!"),
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
			.unwrap(),

		_ => Config::builder()
			.add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml))
			// allow settings from the environment (with a prefix of APP)
			.add_source(Environment::with_prefix("app"))
			.build()
			.unwrap(),
	}
}
