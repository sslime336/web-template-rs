mod api;
mod db;
mod entity;
mod service;

use std::env;

use anyhow::Ok;
use dotenvy::dotenv;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect("`.env` file not found");
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let server_url = format!("{host}:{port}");
    let app = api::router();
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
