use layer_infra::{
    ApiEnvConnector, DatabaseConnector, DatabaseEnvConnector, DatabasePoolConfig, DatabaseUser,
};
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
    let api_env = ApiEnvConnector::new()?;
    let db_env = DatabaseEnvConnector::new(DatabaseUser::Operator)?;
    let db_connector = DatabaseConnector::new(
        db_env.db_user_name,
        db_env.db_user_password,
        db_env.db_host,
        db_env.db_port,
        db_env.db_name,
        Some(DatabasePoolConfig {
            max_connections: db_env.db_pool_max_connections,
            min_connections: db_env.db_pool_min_connections,
            connect_timeout_secs: db_env.db_pool_connect_timeout_secs,
            acquire_timeout_secs: db_env.db_pool_acquire_timeout_secs,
            idle_timeout_secs: db_env.db_pool_idle_timeout_secs,
            max_lifetime_secs: db_env.db_pool_max_lifetime_secs,
        }),
    );
    let address = format!("{}:{}", api_env.host_name_to_bind, api_env.port_to_bind);

    #[allow(unused_mut)]
    let mut allowed_origins: Vec<String> = api_env.allowed_origins;

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
