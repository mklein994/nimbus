#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

#[macro_use]
extern crate clap;
extern crate darksky;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate nimbus;
extern crate pretty_env_logger;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use config::Config;
use config::args;

mod config;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<::std::error::Error>> {
    pretty_env_logger::init().expect("Failed to initialize pretty_env_logger");
    info!("Starting up\u{2026}");

    let m = args::build_cli().get_matches();
    info!("{:#?}", m);

    info!("Retrieving settings\u{2026}");
    let config: Config = Config::load()?;
    info!("{:#?}", config);

    Ok(())
}
