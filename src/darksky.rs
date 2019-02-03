extern crate forecast;
extern crate reqwest;

use crate::client::{Forecast, ForecastService};
use forecast::{
	ApiClient, ApiResponse, ExcludeBlock, ExtendBy, ForecastRequestBuilder, Lang,
	TimeMachineRequestBuilder, Units,
};
use reqwest::Client;

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
}

impl ForecastService for DarkSkyClient {
	fn get_current_weather(&self, latitude: f64, longitude: f64) -> Forecast {
		let api = ApiClient::new(&self.http_client);

		let forecast_req = ForecastRequestBuilder::new(self.api_key.as_str(), latitude, longitude)
			.exclude_block(ExcludeBlock::Hourly)
			.lang(Lang::English)
			.units(Units::Imperial)
			.build();

		let resp = api.get_forecast(forecast_req);

		Forecast {
			current_temp: String::from(""),
		}

		// Forecast { current_temp: response.currently.unwrap().temperature.unwrap() }

		// unimplemented!();
	}
}
