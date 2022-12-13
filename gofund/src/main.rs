use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

mod ui;

fn main() {
    let config = load_configuration();

    ui::create_ui(config);
}

#[derive(Deserialize)]
pub struct Config {
    ip: String,
    port: u16,
}

fn load_configuration() -> Config {
    let contents = fs::read_to_string("config.toml").unwrap_or_else(|err| {
        eprintln!("Error reading configuration file: {}", err);
        exit(1);
    });

    let data: Config = toml::from_str(&contents).unwrap_or_else(|err| {
        eprintln!("Error parsing configuration file: {}", err);
        exit(1);
    });

    data
}
