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
    let path = Path::new(&env::var("HOME").unwrap())
        .join(".config")
        .join("nimbus")
        .join("config.toml");
    info!("{:?}", path.display());
    let mut f = File::open(path).expect("config not found");
    info!("{:?}", f);
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error while reading config");
    debug!("{}", toml::to_string_pretty(&contents).unwrap());
    Ok(toml::from_str(&contents).unwrap())
}

fn main() {
    pretty_env_logger::init().unwrap();

    info!("Starting up…");

    info!("Retrieving settings…");
    let config: Config = if dotenv::from_filename(".secret").is_ok() {
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

        info!("{:#?}", config);
        config
    } else {
        read_config().unwrap()
    };


    if let Err(e) = nimbus::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
