#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"), warn(clippy_pedantic))]

extern crate darksky;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate nimbus;

mod config;

use config::Config;

fn main() {
    pretty_env_logger::init().expect("Failed to initialize pretty_env_logger");
    info!("Starting up\u{2026}");

    info!("Retrieving settings\u{2026}");

    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => panic!("error getting config: {:?}", e),
    };

    info!("{:?}", config);
    // NOTE: END HERE

    /*
    if let Err(e) = nimbus::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
    */
}
