use layer_infra::{DatabaseConnector, DatabasePoolConfig, DatabaseUser, EnvConnector};
use layer_presentation::route;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

/// Run the application server.
async fn run() -> anyhow::Result<()> {
    let env = EnvConnector::new(DatabaseUser::Operator)?;
    let db_connector = DatabaseConnector::new(
        env.db_user_name,
        env.db_user_password,
        env.db_host,
        env.db_port,
        env.db_name,
        Some(DatabasePoolConfig {
            max_connections: env.db_pool_max_connections,
            min_connections: env.db_pool_min_connections,
            connect_timeout_secs: env.db_pool_connect_timeout_secs,
            acquire_timeout_secs: env.db_pool_acquire_timeout_secs,
            idle_timeout_secs: env.db_pool_idle_timeout_secs,
            max_lifetime_secs: env.db_pool_max_lifetime_secs,
        }),
    );
    let address = format!("{}:{}", env.host_name_to_bind, env.port_to_bind);

    #[allow(unused_mut)]
    let mut allowed_origins: Vec<String> = env.allowed_origins;

    #[cfg(feature = "allow-localhost-access")]
    {
        eprintln!("debug feature enabled");
        allowed_origins.push("http://localhost:3000".to_string());
        allowed_origins.push("http://0.0.0.0:3000".to_string());
    }

    tracing_subscriber::fmt::init();

    // run our app with hyper, listening globally on the port
    let listener = tokio::net::TcpListener::bind(address).await?;
    let connection = db_connector.get_connection().await?;
    let router = route(allowed_origins, connection)?;
    axum::serve(listener, router).await?;

    Ok(())
}
