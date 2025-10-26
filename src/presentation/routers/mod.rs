use axum::Router;

mod health;
mod users;

pub fn get_router() -> Router {
  Router::new()
    .nest("/health", health::get_router())
    .nest("/users", users::get_router())
}
