#![allow(unused)]
use axum::routing::get;
use axum::Router;

pub fn config_router() -> Router {
    Router::new().route("/config", get(config_handler))
}

async fn config_handler() -> &'static str {
    "Config handler"
}
