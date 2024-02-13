use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/ping", get(pong))
        .route("/hello", get(hi))
}

async fn hello_world() -> String {
    "Hello, world!".to_owned()
}
async fn pong() {}
async fn hi() {}
