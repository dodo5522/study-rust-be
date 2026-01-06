use axum::{Router, routing::get};

mod getter;

pub fn get_router() -> Router {
    Router::new().route("/", get(getter::check))
}
