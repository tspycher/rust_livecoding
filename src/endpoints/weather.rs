use crate::weather::model::AviationWeather;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

// Handler for the /weather endpoint
#[utoipa::path(
    get,
    path = "/weather",
    tag = "Weather",
    responses(
        (status = 200, description = "Returns the current Weather", body = AviationWeather),
    ),
    security(
        (),
    )
)]
pub async fn weather_handler() -> Result<Json<AviationWeather>, impl IntoResponse> {
    match AviationWeather::fetch().await {
        Ok(weather) => Ok(Json(weather)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch weather data: {:?}", e),
        )),
    }
}
