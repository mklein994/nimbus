#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate darksky;
extern crate reqwest;
extern crate serde_json;
extern crate weather_icons;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

//use darksky::DarkskyReqwestRequester;
use darksky::models::Icon;
use weather_icons::WeatherIcon;
use weather_icons::moon;
//use reqwest::Client;

pub fn run() -> Result<(), Box<Error>> {
    get_weather()
}

fn get_weather() -> Result<(), Box<Error>> {
    let mut f = File::open("/home/matthew/projects/weather/tests/data/forecast.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let weather_json: darksky::models::Forecast = serde_json::from_str(&contents)?;

    print!("<span font_desc='Weather Icons'>");
    for i in weather_json.daily.unwrap().data.unwrap() {
        let current_phase = i.moon_phase.unwrap();
        print!("{}", moon::phase(current_phase));
    }

    println!(
        " {}</span>",
        get_icon(weather_json.currently.unwrap().icon.unwrap())
    );
    Ok(())
}

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
