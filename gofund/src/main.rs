use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

use cursive::views::Dialog;

fn main() {
    let config = load_configuration();

    let mut siv = cursive::default();

    siv.set_window_title("Goindol Fund Client");

    siv.add_layer(
        Dialog::text(format!(
            "Server ip: {}\nServer port: {}",
            &config.ip, &config.port
        ))
        .title("Information")
        .button("OK", |s| s.quit()),
    );

    siv.run();
}

#[derive(Deserialize)]
struct Config {
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
