use dotenvy::var;

use super::get_router;

/// Run the application server.
pub async fn run() -> anyhow::Result<()> {
    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let router = get_router();
    let listener = tokio::net::TcpListener::bind(format!("{bind_addr}:{bind_port}")).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
