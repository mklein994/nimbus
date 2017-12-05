#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

#[macro_use]
extern crate clap;
extern crate darksky;
#[macro_use]
extern crate log;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate weather_icons;

mod config;

use config::{args, Config};
use darksky::DarkskyReqwestRequester;
use darksky::models::{Datablock, Datapoint};
use darksky::models::Icon as DarkskyIcon;
use reqwest::Client;
use std::error;
use std::fmt;
use std::io;
use std::io::prelude::*;
use weather_icons::Icon;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Darksky(darksky::Error),
    Config(config::Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Darksky(ref err) => err.fmt(f),
            Error::Config(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Darksky(ref err) => err.description(),
            Error::Config(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Darksky(ref err) => Some(err),
            Error::Config(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<darksky::Error> for Error {
    fn from(err: darksky::Error) -> Self {
        Error::Darksky(err)
    }
}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Self {
        Error::Config(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

#[derive(Debug)]
struct Currently {
    icon: Icon,
    summary: String,
    temperature: f64,
    pressure: f64,
}

impl From<Datapoint> for Currently {
    fn from(datapoint: Datapoint) -> Self {
        Self {
            icon: get_icon(datapoint.icon.expect("current icon missing")),
            summary: datapoint.summary.expect("current summary missing"),
            temperature: datapoint.temperature.expect("current temperature missing"),
            pressure: datapoint.pressure.expect("current pressure missing"),
        }
    }
}

impl fmt::Display for Currently {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{icon} {temperature}\u{00b0} {summary}",
            icon = self.icon,
            temperature = self.temperature,
            summary = self.summary
        )
    }
}

impl Default for Currently {
    fn default() -> Self {
        Self {
            icon: Icon::Na,
            temperature: 0.0,
            pressure: 0.0,
            summary: String::from("Weather not available"),
        }
    }
}

#[derive(Debug)]
struct Daily {
    icon: Icon,
    summary: String,
}

impl fmt::Display for Daily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{icon} {summary}",
            icon = self.icon,
            summary = self.summary
        )
    }
}

impl From<Datablock> for Daily {
    fn from(datablock: Datablock) -> Self {
        Self {
            icon: get_icon(datablock.icon.expect("daily icon missing")),
            summary: datablock.summary.expect("daily summary missing"),
        }
    }
}

fn get_weather(config: Config) -> Result<darksky::models::Forecast> {
    if let Some(f) = config.local {
        let mut forecast = std::fs::File::open(f)?;
        let mut contents = String::new();
        forecast.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(Error::Json)
    } else {
        let client = Client::new();

        client
            .get_forecast(
                &config.token,
                config.coordinates.latitude,
                config.coordinates.longitude,
            )
            .map_err(Error::Darksky)
    }
}

fn get_icon(icon: DarkskyIcon) -> Icon {
    match icon {
        DarkskyIcon::ClearDay => Icon::DarkskyClearDay,
        DarkskyIcon::ClearNight => Icon::DarkskyClearNight,
        DarkskyIcon::Cloudy => Icon::DarkskyCloudy,
        DarkskyIcon::Fog => Icon::DarkskyFog,
        DarkskyIcon::Hail => Icon::DarkskyHail,
        DarkskyIcon::PartlyCloudyDay => Icon::DarkskyPartlyCloudyDay,
        DarkskyIcon::PartlyCloudyNight => Icon::DarkskyPartlyCloudyNight,
        DarkskyIcon::Rain => Icon::DarkskyRain,
        DarkskyIcon::Sleet => Icon::DarkskySleet,
        DarkskyIcon::Snow => Icon::DarkskySnow,
        DarkskyIcon::Thunderstorm => Icon::DarkskyThunderstorm,
        DarkskyIcon::Tornado => Icon::DarkskyTornado,
        DarkskyIcon::Wind => Icon::DarkskyWind,
    }
}

pub fn run() -> Result<()> {
    let m = args::build_cli().get_matches();
    info!("{:#?}", m);

    info!("Retrieving settings\u{2026}");
    let config: Config = Config::load()?;
    info!("{:#?}", config);

    let weather = get_weather(config);

    info!("{}", weather.unwrap().currently.unwrap().summary.unwrap());

    Ok(())
}
