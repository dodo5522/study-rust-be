use axum::{Router, routing::{get, post}};

mod models;
mod creator;
mod getter;

pub fn get_router() -> Router {
  Router::new()
    .route("/", post(creator::create))
    .route("/", get(getter::get))
}
