//! Entry point for the Config API service.

use actix_web::{App, HttpServer};
use config_api::{build_state, configure_routes};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let state = build_state();

    let port = 8080_u16;
    info!("Config API listening on port {port}");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
