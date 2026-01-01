use axum::{
    Router,
    routing::{get, post},
};

mod creator;
mod getter;
mod models;

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(creator::create))
        .route("/", get(getter::get))
}
