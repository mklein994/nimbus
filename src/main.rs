extern crate darksky;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate nimbus;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::env;
use toml::Value;

#[derive(Debug, Deserialize)]
struct Config {
    coordinates: Coordinates,
    token: String,
    language: Option<darksky::Language>,
    unit: Option<darksky::Unit>,
}

impl<'a> Config {
    fn new() -> Config {
        Config {
            coordinates: Coordinates {
                latitude: 49.9,
                longitude: -97.0,
            },
            token: "".to_string(),
            language: Some(darksky::Language::En),
            unit: Some(darksky::Unit::Ca),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

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

        let config = Config::new();
        info!("{:#?}", config);
        config
    } else {
        read_config().unwrap()
    };


    if let Err(e) = nimbus::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
