#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

extern crate toml;
extern crate xdg;

use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

type Result<T> = ::std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Xdg(xdg::BaseDirectoriesError),
    Toml(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Io(ref err) => write!(f, "IO error: {}", err),
            ConfigError::Xdg(ref err) => write!(f, "Xdg error: {}", err),
            ConfigError::Toml(ref err) => write!(f, "Toml error: {}", err),
        }
    }
}

impl error::Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Io(ref err) => err.description(),
            ConfigError::Xdg(ref err) => err.description(),
            ConfigError::Toml(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ConfigError::Io(ref err) => Some(err),
            ConfigError::Xdg(ref err) => Some(err),
            ConfigError::Toml(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<xdg::BaseDirectoriesError> for ConfigError {
    fn from(err: xdg::BaseDirectoriesError) -> ConfigError {
        ConfigError::Xdg(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> ConfigError {
        ConfigError::Toml(err)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub coordinates: Coordinates,
    pub token: String,
    /// darksky::Language
    pub language: Option<String>,
    /// darksky::Unit
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = match config_path() {
            Some(c) => c,
            None => panic!("Config error"),
        };

        info!("{:?}", config.display());

        let mut f = File::open(config).map_err(ConfigError::Io)?; //.expect("config not found");
        info!("{:?}", f);

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        debug!(
            "{}",
            toml::to_string_pretty(&contents).expect("Failed to prettify config")
        );

        Ok(toml::from_str(&contents).expect("Failed to parse config"))
    }
}

fn config_path() -> Option<PathBuf> {
    xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
        .expect("Can't find ~/.config/weather/")
        .find_config_file(Path::new("config.toml"))
}
