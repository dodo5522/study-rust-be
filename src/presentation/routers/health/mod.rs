use axum::{routing::get, Router};

mod getter;


pub fn get_router() -> Router {
  Router::new().route("/", get(getter::check))
}
