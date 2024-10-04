use std::fmt;
use std::time::Duration;
use reqwest::Client;
// here goes our model
/*
API Response
{
  "icao": "LSZI",
  "webcam": {
    "name": "Flugplatz Fricktal Schupfart",
    "image_url": "https://storage.roundshot.com/58eb4481605615.63926737/2024-09-06/15-30-00/2024-09-06-15-30-00_full.jpg"
  },
  "runway": {
    "status": 3,
    "altitude": 1788,
    "text": "Flugplatz normal benutzbar",
    "additional": "nothing"
  },
  "weather": {
    "weatherlink": null,
    "meteomedia": {
      "FF": 4,
      "wind_mean": 3,
      "wind_dir_mean": 118,
      "wind_max": 4,
      "MM": 9,
      "DD": 6,
      "T5": "/",
      "VIS": "/",
      "HHmm": "1330",
      "GL10": 368,
      "RR10m": 0,
      "YYYY": 2024,
      "DIR": 70,
      "NAME": "Schupfart",
      "TD": 17,
      "QFF": 1010.8,
      "QFE": "/",
      "RR1h": "/",
      "Sh": "/",
      "ELEV": 547,
      "TL": 23.3,
      "GL1h": "/",
      "G1h": "/"
    },
    "aviation": {
      "oat": 23.299999237060547,
      "valid": true,
      "timestamp": 1725629400,
      "age": 560,
      "alt": 1794,
      "dew": 17,
      "spread": 6.299999237060547,
      "hpa": 1010.7999877929688,
      "humidity": 0,
      "wind_kmh": 5.556,
      "wind_kt": 3,
      "gust_kmh": 7.408,
      "gust_kt": 4,
      "wind_dir": 70,
      "pa": 1865.7129518568233,
      "da": 3292.4215355040888,
      "cloud_base": 3226,
      "rain_rate_mm": 0
    }
  },
  "weather_timestamp": 1725629400,
  "runway_timestamp": 1725629961,
  "webcam_timestamp": 1725629961,
  "age": 0
}
 */
use serde::{Deserialize, Serialize};
use tracing::info;

// the root of the API response
#[derive(Deserialize, Debug)]
struct ApiResponse {
    weather: Weather,
}

// the first level of the response
#[derive(Deserialize, Debug)]
struct Weather {
    aviation: AviationWeather,
}

// the actual weather data
#[derive(Serialize, Deserialize, Debug)]
pub struct AviationWeather {
    oat: f32,
    valid: bool,
    timestamp: u64,
    age: i32,
    alt: i32,
    dew: f64,
    spread: f32,
    hpa: f64,
    humidity: f64,
    wind_kmh: f64,
    wind_kt: f64,
    gust_kmh: f64,
    gust_kt: f64,
    wind_dir: f64,
    pa: f64,
    da: f64,
    cloud_base: i32,
    rain_rate_mm: f64,
}
// Implementing the Display trait for AviationWeather
impl fmt::Display for AviationWeather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the aviation weather data into a nicely printed format
        write!(
            f,
            "Aviation Weather Report:\n\
             ----------------------------\n\
             Outside Air Temperature (OAT): {:.1}°C\n\
             Valid: {}\n\
             Altitude: {} ft\n\
             Dew Point: {}°C\n\
             Temperature Spread: {:.1}°C\n\
             Atmospheric Pressure: {:.1} hPa\n\
             Humidity: {}%\n\
             Wind Speed: {:.1} km/h ({} kt)\n\
             Wind Direction: {}°\n\
             Gust Speed: {:.1} km/h ({} kt)\n\
             Cloud Base: {} ft\n\
             Rain Rate: {} mm/h\n\
             Pressure Altitude (PA): {:.2} ft\n\
             Density Altitude (DA): {:.2} ft\n\
             Timestamp: {}\n\
             Age: {} seconds",
            self.oat,
            self.valid,
            self.alt,
            self.dew,
            self.spread,
            self.hpa,
            self.humidity,
            self.wind_kmh,
            self.wind_kt,
            self.wind_dir,
            self.gust_kmh,
            self.gust_kt,
            self.cloud_base,
            self.rain_rate_mm,
            self.pa,
            self.da,
            self.timestamp,
            self.age
        )
    }
}


impl AviationWeather {
    pub async fn fetch() -> Result<Self, reqwest::Error> {
        // Build the HTTP client
        let client = Client::builder()
            .timeout(Duration::from_secs(10)) // Optional: Set a timeout for the request
            .build()?;

        // API endpoint
        let url = "https://towpilot.ormalingen.tspycher.com/airfields/lszi";
        info!("Fetching aviation weather data from {}", url);

        // Fetch and deserialize the data
        let response = client.get(url).send().await?.json::<ApiResponse>().await?;

        // Return the aviation weather data
        Ok(response.weather.aviation)
    }
}

#[tokio::test]
async fn test_fetch() {
    let weather = AviationWeather::fetch().await.unwrap();

    assert_eq!(weather.valid, true);
    assert!(weather.alt > 0);

    println!("{}", weather);
}
