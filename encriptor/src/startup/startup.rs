use crate::models::config::Config;
use std::fs;

pub fn read_config() -> Config {
    let config_contents = fs::read_to_string("config.toml").expect("Failed to read config file");

    toml::from_str(&config_contents).expect("Failed to parse config")
}
