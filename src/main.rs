use dotenvy::dotenv;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use sea_orm::Database;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod entity;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load .env
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // get DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // connect SeaORM Database
    let db = Database::connect(&database_url).await?;
    tracing::info!("Connected to database");

    // build app routes
    let app = Route::new()
        .at("/health", poem::get(|| async { "ok" }))
        .at("/users", poem::get(routes::list_users).post(routes::create_user))
        // share db with handlers
        .data(db);

    let addr = "127.0.0.1:3000";
    tracing::info!("Listening on http://{}", addr);
    Server::new(TcpListener::bind(addr)).run(app).await?;

    Ok(())
}