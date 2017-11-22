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

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Xdg(xdg::BaseDirectoriesError),
    Toml(toml::de::Error),
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Xdg(ref err) => err.fmt(f),
            Error::Toml(ref err) => err.fmt(f),
            Error::NotFound => write!(
                f,
                "Configuration file not found."
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Xdg(ref err) => err.description(),
            Error::Toml(ref err) => err.description(),
            Error::NotFound => "not found",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Xdg(ref err) => Some(err),
            Error::Toml(ref err) => Some(err),
            Error::NotFound => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<xdg::BaseDirectoriesError> for Error {
    fn from(err: xdg::BaseDirectoriesError) -> Self {
        Error::Xdg(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
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
        let config = config_path()?;

        info!("{:?}", config.display());

        let mut contents = String::new();
        File::open(config).and_then(|mut f| f.read_to_string(&mut contents))?;

        Ok(toml::from_str(&contents)?)
    }
}

fn config_path() -> Result<PathBuf> {
    xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?
        .find_config_file(Path::new("config.toml"))
        .ok_or(Error::NotFound)
}
