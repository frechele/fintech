use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub dbname: String,
    pub user: String,
    pub password: String,
}

pub fn load_configuration(filename: &str) -> Config {
    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading configuration file: {}", err);
        exit(1);
    });

    let config: Config = toml::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        exit(1);
    });

    config
}
