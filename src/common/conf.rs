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
