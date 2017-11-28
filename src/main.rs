#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

extern crate darksky;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;

extern crate nimbus;

fn main() {
    pretty_env_logger::init().expect("Failed to initialize pretty_env_logger");
    info!("Starting up\u{2026}");

    if let Err(e) = nimbus::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
