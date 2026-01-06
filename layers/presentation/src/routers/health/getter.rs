#[allow(unused)]
use poem::{
    Route, Server, delete, get, handler, listener::TcpListener, middleware::Tracing, post, put,
    web::Path,
};

#[allow(unused)]
use layer_infra_db::repository::test;

pub async fn check() -> &'static str {
    "I'm alive."
}
