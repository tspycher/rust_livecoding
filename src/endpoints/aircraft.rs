use axum::extract::State;
use axum::response::IntoResponse;
use diesel::prelude::*;

use tokio::sync::Mutex;
use crate::schema::aircraft::dsl::*;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::Json;
use crate::appstate::AppState;
use crate::models::aircraft::{Aircraft, NewAircraft};

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
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Aircraft>>, impl IntoResponse> {

    let s = state.lock().await;
    let pool = s.pool.lock().await;
    let mut conn = pool.get().expect("Failed to get connection from pool.");

    match aircraft.load::<Aircraft>(&mut conn) {
        Ok(aircrafts) => Ok(Json(aircrafts)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error listing aircrafts: {}", e)))
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
    State(state): State<Arc<Mutex<AppState>>>,
    Json(new_aircraft): Json<NewAircraft>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let s = state.lock().await;
    let pool = s.pool.lock().await;
    let mut conn = pool.get().expect("Failed to get connection from pool.");

    // Insert the new aircraft into the database
    match diesel::insert_into(aircraft)
        .values(&new_aircraft)
        .execute(&mut conn) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error creating aircraft: {}", e)))
    }
}

