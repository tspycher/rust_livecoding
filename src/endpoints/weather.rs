use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::weather::model::AviationWeather;

// Handler for the /weather endpoint
pub(crate) async fn weather_handler() -> Result<Json<AviationWeather>, impl IntoResponse> {
    match AviationWeather::fetch().await {
        Ok(weather) => Ok(Json(weather)),
        Err(e) =>  Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch weather data: {:?}", e)))
    }
}
