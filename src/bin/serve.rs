use dotenvy::var;

use study_rust_be::get_router;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

/// Run the application server.
async fn run() -> anyhow::Result<()> {
    let bind_addr = var("BIND_ADDR")?;
    let bind_port = var("BIND_PORT")?;

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let router = get_router();
    let listener = tokio::net::TcpListener::bind(format!("{bind_addr}:{bind_port}")).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
