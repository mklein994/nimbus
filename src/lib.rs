#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(clippy_pedantic)]

extern crate darksky;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate weather_icons;

use darksky::models::{Datablock, Datapoint, Icon};
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use weather_icons::WeatherIcon;

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
struct CurrentWeather {
    icon: WeatherIcon,
    summary: String,
    temperature: f64,
    pressure: f64,
}

impl From<Datapoint> for CurrentWeather {
    fn from(datapoint: Datapoint) -> Self {
        Self {
            icon: get_icon(datapoint.icon.expect("current icon missing")),
            summary: datapoint.summary.expect("current summary missing"),
            temperature: datapoint.temperature.expect("current temperature missing"),
            pressure: datapoint.pressure.expect("current pressure missing"),
        }
    }
}

impl fmt::Display for CurrentWeather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{icon} {temperature}{unit} {summary}",
            icon = self.icon,
            temperature = self.temperature,
            unit = format!("\u{00b0}{}", "C"), // FIXME: use the unit passed to darksky::Options
            summary = self.summary
        )
    }
}

impl Default for CurrentWeather {
    fn default() -> Self {
        Self {
            icon: WeatherIcon::Na,
            temperature: 0.0,
            pressure: 0.0,
            summary: String::from("Weather not available"),
        }
    }
}

#[derive(Debug)]
struct DailyWeather {
    icon: WeatherIcon,
    summary: String,
}

impl fmt::Display for DailyWeather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{icon} {summary}",
            icon = self.icon,
            summary = self.summary
        )
    }
}

impl From<Datablock> for DailyWeather {
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

    info!(
        "{:?}",
        weather
            .flags
            .expect("Missing weather flags")
            .units
            .expect("Error getting flag units")
    );

    //debug!("{:?}", weather);

    let current_json = weather.currently.unwrap_or_else(|| {
        panic!("Error getting current weather");
    });

    let current = CurrentWeather::from(current_json);
    debug!("current: {:#?}", current);
    info!("current: {}", current);

    let daily_json = weather.daily.unwrap_or_else(|| {
        panic!("daily weather missing");
    });

    let daily = DailyWeather::from(daily_json);
    debug!("daily: {:#?}", daily);
    info!("daily: {}", daily);

    Ok(())
}

fn get_weather() -> Result<darksky::models::Forecast, Box<Error>> {
    let mut f = File::open("/home/matthew/projects/weather/tests/data/forecast_2.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let weather_json: darksky::models::Forecast = serde_json::from_str(&contents)?;

    Ok(weather_json)
}

/*
fn current_weather(weather: &darksky::models::Forecast) -> Result<CurrentWeather, Box<Error>> {
    let currently = weather.currently.unwrap();
    debug!("currently: {:#?}", currently);

    let summary = currently.summary.expect("currently: summary missing");
    info!("currently.summary: {}", summary);

    let icon = get_icon(currently.icon.expect("currently: icon missing"));
    info!("currently.icon: {}", icon);
    debug!("currently.icon: {:?}", icon);

    let temperature = currently
        .temperature
        .expect("currently: temperature missing");
    info!("currently.temperature: {}", temperature);

    let pressure = currently.pressure.expect("currently: missing pressure");
    info!("currently.pressure: {}", pressure);

    let unit = weather.flags.unwrap().units.unwrap();

    Ok(CurrentWeather {
        icon,
        summary,
        temperature,
        pressure,
        unit,
    })
}
*/

fn get_icon(icon: Icon) -> WeatherIcon {
    match icon {
        Icon::ClearDay => WeatherIcon::DarkskyClearDay,
        Icon::ClearNight => WeatherIcon::DarkskyClearNight,
        Icon::Cloudy => WeatherIcon::DarkskyCloudy,
        Icon::Fog => WeatherIcon::DarkskyFog,
        Icon::Hail => WeatherIcon::DarkskyHail,
        Icon::PartlyCloudyDay => WeatherIcon::DarkskyPartlyCloudyDay,
        Icon::PartlyCloudyNight => WeatherIcon::DarkskyPartlyCloudyNight,
        Icon::Rain => WeatherIcon::DarkskyRain,
        Icon::Sleet => WeatherIcon::DarkskySleet,
        Icon::Snow => WeatherIcon::DarkskySnow,
        Icon::Thunderstorm => WeatherIcon::DarkskyThunderstorm,
        Icon::Tornado => WeatherIcon::DarkskyTornado,
        Icon::Wind => WeatherIcon::DarkskyWind,
    }
}
