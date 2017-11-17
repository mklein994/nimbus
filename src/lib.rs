#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

extern crate darksky;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate weather_icons;

use darksky::models::{Datablock, Datapoint};
use darksky::models::Icon as DarkskyIcon;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use weather_icons::Icon;

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
