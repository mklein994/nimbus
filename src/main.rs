#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"), warn(clippy_pedantic))]

extern crate darksky;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;
extern crate nimbus;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use nimbus::{Config, Coordinates};

fn read_config() -> Result<Config, Box<std::error::Error>> {
    let path = Path::new(&env::var("HOME")
        .expect("HOME environment variable not defined"))
        .join(".config")
        .join("nimbus")
        .join("config.toml");
    info!("{:?}", path.display());

    let mut f = File::open(path).expect("config not found");
    info!("{:?}", f);

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error while reading config");
    debug!(
        "{}",
        toml::to_string_pretty(&contents).expect("Failed to prettify config")
    );

    Ok(toml::from_str(&contents).expect("Failed to parse config"))
}

fn main() {
    pretty_env_logger::init().expect("Failed to initialize pretty_env_logger");

    info!("Starting up\u{2026}");

    info!("Retrieving settings\u{2026}");

    let config: Config = if dotenv::dotenv().is_ok() {
        info!("Settings from dotenv");

        let token = env::var("DARKSKY_KEY").expect(
            "Missing DARKSKY_KEY. Please set the DARKSKY_KEY environment variable to your API key",
        );

        let latitude = env::var("DARKSKY_LAT")
            .expect("Missing DARKSKY_LAT")
            .parse::<f64>()
            .expect("latitude must be a floating point number");

        let longitude = env::var("DARKSKY_LON")
            .expect("Missing DARKSKY_LON")
            .parse::<f64>()
            .expect("longitude must be a floating point number");

        let coordinates = Coordinates {
            latitude: latitude,
            longitude: longitude,
        };

        let config = Config {
            coordinates,
            token,
            language: Some("en".to_string()),
            unit: Some("ca".to_string()),
        };

        debug!("{:#?}", config);
        config
    } else {
        info!("Settings from config");
        read_config().expect("Couldn't read config")
    };


    if let Err(e) = nimbus::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
