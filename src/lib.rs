#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"), feature(plugin))]
#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"), plugin(clippy))]
#![cfg_attr(any(feature = "clippy", feature = "cargo-clippy"), warn(clippy_pedantic))]

extern crate darksky;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate weather_icons;

use darksky::models::{Datablock, Datapoint};
use darksky::models::Icon as DarkskyIcon;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use weather_icons::Icon;

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

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let weather = get_weather().unwrap_or_else(|e| {
        panic!("Error getting weather: {:?}", e);
    });

    let current_json = weather.currently.expect("Error getting current weather");

    let current = Currently::from(current_json);
    debug!("current: {:#?}", current);
    info!("current: {}", current);

    let daily_json = weather.daily.expect("daily weather missing");

    let daily = Daily::from(daily_json);
    debug!("daily: {:#?}", daily);
    info!("daily: {}", daily);

    Ok(())
}

fn get_weather() -> Result<darksky::models::Forecast, Box<Error>> {
    // TODO: Actually get the weather from the Internet.
    debug!("CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));
    let mut f =
        File::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/forecast_2.json"))?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let weather_json: darksky::models::Forecast = serde_json::from_str(&contents)?;

    Ok(weather_json)
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
