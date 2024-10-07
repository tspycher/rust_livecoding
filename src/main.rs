use axum::routing::get;
use axum::Router;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use std::env;
use std::sync::Arc;
use tedtalk::apidoc::ApiDoc;
use tedtalk::appstate::AppState;
use tedtalk::endpoints;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[tokio::main]
async fn main() {
    // Set up the logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // get some information from the Cargo.toml
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    info!("Starting {name} API Server version {version}");

    // establish the database connection
    let database_url = env::var("DATABASE_URL").unwrap_or("db.sqlite".to_string());
    info!("Connecting to database: {}", database_url);
    let manager = ConnectionManager::<SqliteConnection>::new("db.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let appstate = AppState {
        pool: Arc::new(pool),
    };

    // Build the Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hi Wilmaa Team, time to get rusty!" }))
        .route("/weather", get(endpoints::weather::weather_handler))
        .route(
            "/aircrafts",
            get(endpoints::aircraft::list_aircraft).post(endpoints::aircraft::create_aircraft),
        )
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
        .with_state(Arc::new(appstate));

    // Run the application on localhost:3000
    let host = "127.0.0.1";
    let port = 3000;
    let addr = format!("{}:{}", host, port);
    info!("Starting server on: http://{}", addr);
    info!("Docs are served on: http://{}/docs", addr);

    // Run the application
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
