use std::env;

use config::Config;

pub fn authors_file() -> String {
    return get_config().get::<String>("authors_file").unwrap();
}

fn get_config() -> Config {
    let mut config_file = "default";
    if let Ok(test_env) = env::var("COA_ENV") {
        if test_env == "test" {
            config_file = "test";
        }
    }
    return Config::builder()
        .add_source(config::File::with_name(
            format!("src/conf/src/configs/{}", config_file).as_str(),
        ))
        .build()
        .unwrap();
}
