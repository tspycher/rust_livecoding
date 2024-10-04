use axum::Router;
use axum::routing::get;


mod weather;
mod endpoints;

#[tokio::main]
async fn main() {
    // Build the Axum router
    let app = Router::new()
        .route("/weather", get(endpoints::weather::weather_handler));

    // Run the application on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

