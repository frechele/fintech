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
    let contents = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read config file");
            exit(-1);
        }
    };

    let data: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load config file");
            exit(-1);
        }
    };

    data
}
