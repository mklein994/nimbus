extern crate darksky;
extern crate reqwest;

use darksky::DarkskyReqwestRequester;
use reqwest::Client;

pub fn run() {
    get_weather();
}

fn get_weather() {
    println!("Hello…");
}
