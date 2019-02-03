pub trait ForecastService {
	fn get_current_weather(&self, latitude: f64, longitude: f64) -> Forecast;
}

pub struct Forecast {
	pub current_temp: String,
}