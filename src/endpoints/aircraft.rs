use axum::extract::State;
use axum::response::IntoResponse;
use diesel::prelude::*;

use crate::appstate::AppState;
use crate::models::aircraft::{Aircraft, NewAircraft};
use crate::schema::aircraft::dsl::*;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/aircrafts",
    tag = "Aircraft",
    responses(
        (status = 200, description = "Lists all Aircrafts", body = [Aircraft]),
    ),
    security(
        (),
    )
)]
pub async fn list_aircraft(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Aircraft>>, impl IntoResponse> {
    let mut conn = state.pool.get().expect("Failed to get connection from pool.");

    match aircraft.load::<Aircraft>(&mut conn) {
        Ok(aircrafts) => Ok(Json(aircrafts)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error listing aircrafts: {}", e),
        )),
    }
}

#[utoipa::path(
    post,
    path = "/aircrafts",
    tag = "Aircraft",
    responses(
        (status = 200, description = "Create a new Aircraft"),
    ),
    security(
        (),
    )
)]
pub async fn create_aircraft(
    State(state): State<Arc<AppState>>,
    Json(new_aircraft): Json<NewAircraft>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut conn = state.pool.get().expect("Failed to get connection from pool.");

    // Insert the new aircraft into the database
    match diesel::insert_into(aircraft)
        .values(&new_aircraft)
        .execute(&mut conn)
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating aircraft: {}", e),
        )),
    }
}
