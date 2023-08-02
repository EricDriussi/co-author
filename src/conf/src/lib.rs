use std::env;

use config::Config;

pub fn authors_file() -> String {
    return get_config().get::<String>("authors_file").unwrap();
}

fn get_config() -> Config {
    let mut config_file = "src/conf/src/configs/default";
    if let Ok(test_env) = env::var("COA_ENV") {
        if test_env == "test" {
            config_file = "../conf/src/configs/test";
        }
    }
    return Config::builder()
        .add_source(config::File::with_name(config_file))
        .build()
        .unwrap();
}
