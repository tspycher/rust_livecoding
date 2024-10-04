use axum::Router;
use axum::routing::get;
use tracing::{info, Level};
use tracing_subscriber::{FmtSubscriber};


mod weather;
mod endpoints;

#[tokio::main]
async fn main() {
    // Set up the logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // get some information from the Cargo.toml
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    info!("Starting {name} API Server version {version}");

    // Build the Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hi Wilmaa Team, time to get rusty!" }))
        .route("/weather", get(endpoints::weather::weather_handler));

    // Run the application on localhost:3000
    let host = "127.0.0.1";
    let port = 3000;
    let addr = format!("{}:{}", host, port);
    info!("Starting server on: http://{}", addr);

    // Run the application
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

