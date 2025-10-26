use super::get_router;

/// Run the application server.
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let router = get_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
