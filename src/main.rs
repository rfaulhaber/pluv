extern crate clap;
extern crate dirs;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
use clap::{App, Arg};
use dirs::home_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

mod client;

use client::client::{ClientType, Forecast, ForecastClient};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    service: String,
    api_key: String,
    latitude: String,
    longitude: String,
}

fn main() {
    let home_path = match home_dir() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => fatalln("could not find home directory"),
    };

    let default_config_path = Path::new(home_path.as_str()).join(".pluvrc");

    let matches = App::new("pluv")
        .version("0.1.0")
        .about("weather app for the terminal")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("filepath")
                .help("path to config file")
                .default_value(default_config_path.to_str().unwrap()),
        )
        .get_matches();

    let file = match File::open(&default_config_path) {
        Ok(file) => file,
        Err(why) => {
            fatalln(format!("could not open {}: {}", default_config_path.display(), why).as_str());
        }
    };

    let config: Config = match serde_yaml::from_reader(&file) {
        Ok(result) => result,
        Err(err) => fatalln(format!("something went wrong: {}", err).as_str()),
    };

    let client_type = match config.service.as_str() {
        "DarkSky" => ClientType::DarkSky {
            api_key: config.api_key,
        },
        _ => panic!("invalid service type"),
    };

    let mut service = client::client::new_client(client_type);

    println!(
        "{}",
        service.get_current_weather(
            config.latitude.parse().unwrap(),
            config.longitude.parse().unwrap()
        )
    );
}

// fn get_current_weather(service: ForecastService) -> String {
// 	return service.get_current_weather();
// }

fn fatalln(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1);
}
