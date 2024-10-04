use crate::endpoints;
use crate::weather;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
        info(
            title = "Rust Live Coding Session",
            description = "Some livecoding Session with Tom",
            contact(
                name = "Thomas Spycher",
                url = "https://tspycher.com",
                email = "me@tspycher.com"
            ),
            license(
                name = "MIT",
                url = "https://opensource.org/license/mit"
            ),
        ),
        tags(
            (name = "Weather", description = "Get Aviation Weather"),
            (name = "Aircraft", description = "Manage Aircrafts"),
        ),
        paths(
            endpoints::weather::weather_handler,
            endpoints::aircraft::list_aircraft,
            endpoints::aircraft::create_aircraft
        ),
        components(
            schemas(
                weather::model::AviationWeather,
                crate::models::aircraft::Aircraft,
                crate::models::aircraft::NewAircraft,
            ),
        ),
        security(
            (),
        ),
        servers(
            (url="http://127.0.0.1:3000", description="Local Development"),
        )
    )]
pub struct ApiDoc;
