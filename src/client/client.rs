use super::darksky::DarkSkyClient;
use std::error;
use std::fmt;

pub enum ClientType {
	DarkSky { api_key: String },
}

pub enum ForecastClient {
	DarkSkyClient(DarkSkyClient),
}

impl ForecastClient {
	pub fn get_current_weather(&mut self, latitude: f64, longitude: f64) -> Forecast {
		match self {
			ForecastClient::DarkSkyClient(ds) => ds.get_current_weather(latitude, longitude),
		}
	}
}

#[derive(Debug)]
pub struct Forecast {
	pub current_temp: String,
}

// TODO implement better display!
impl std::fmt::Display for Forecast {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "current temperature: {}", self.current_temp)
	}
}

pub fn new_client(clientType: ClientType) -> ForecastClient {
	match clientType {
		ClientType::DarkSky { api_key: key } => {
			ForecastClient::DarkSkyClient(DarkSkyClient::new(key))
		}
	}
}
