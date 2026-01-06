#[allow(unused)]
use poem::{EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing};

use axum::Router;

mod health;
mod users;

pub fn get_router() -> Router {
    Router::new()
        .nest("/health", health::get_router())
        .nest("/users", users::get_router())
}
