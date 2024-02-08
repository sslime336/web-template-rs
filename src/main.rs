use std::time::Duration;

use sea_orm::{ConnectOptions, Database};
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, world!");
    warn!("Hello, world!");

    let mut opt = ConnectOptions::new("sqlite://.local/mydb.sqlite?mode=rwc");

    let db = Database::connect(opt).await.unwrap();
    
}
