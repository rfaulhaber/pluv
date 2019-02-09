extern crate reqwest;
extern crate serde_json;

use super::client::Forecast;
use reqwest::Client;
use serde_json::Value;

static DEFAULT_ENDPOINT: &str = "https://api.darksky.net/forecast";

pub struct DarkSkyClient {
	api_key: String,
	http_client: Client,
}

impl DarkSkyClient {
	pub fn new(api_key: String) -> DarkSkyClient {
		DarkSkyClient {
			api_key,
			http_client: Client::new(),
		}
	}

	pub fn get_current_weather(&mut self, latitude: f64, longitude: f64) -> Forecast {
		let resp = self
			.http_client
			.get(self.get_endpoint(latitude, longitude).as_str())
			.send();

		// TODO return result<forecast, error>

		let resp_str = match resp {
			Ok(mut r) => match r.text() {
				Ok(str_r) => str_r,
				Err(e) => panic!("error: {}", e),
			},
			// TODO remove panic
			Err(e) => panic!("error: {}", e),
		};

		let v: Value = serde_json::from_str(resp_str.as_str()).unwrap();

		Forecast {
			current_temp: format!("{}", v["currently"]["temperature"]),
		}
	}

	fn get_endpoint(&self, latitude: f64, longitude: f64) -> String {
		vec![
			DEFAULT_ENDPOINT,
			self.api_key.as_str(),
			format!("{},{}", latitude, longitude).as_str(),
		]
		.join("/")
	}
}
