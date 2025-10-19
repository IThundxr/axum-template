mod app;
mod error;

use std::env;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Loading .env");
    dotenvy::dotenv()?;

    log::debug!("Configuring tracing subscriber from env");
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let app = App::new();

    let router = Router::new()
        .route("/status", get(|| async { StatusCode::OK }))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(app));

    let ip = env::var("APP_IP").unwrap_or("0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or("3000".to_string());
    let address = format!("{ip}:{port}");

    log::info!("Listening on {address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
