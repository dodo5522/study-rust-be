use layer_domain::{EnergyRecord, IGenerationRepository};
use layer_infra_db::get_connection;
use layer_infra_db::repository::generation::GenerationRepository;
use std::env::var;

#[allow(unused)]
use poem::{
    Route, Server, delete, get, handler, listener::TcpListener, middleware::Tracing, post, put,
    web::Path,
};

pub async fn check() -> &'static str {
    let user_ = var("DB_OPERATOR_NAME");
    let password_ = var("DB_OPERATOR_PASSWORD");

    if let Ok(user) = user_
        && let Ok(password) = password_
        && let Ok(db) = get_connection(&user, &password).await
        && let Ok(repo) = GenerationRepository::new(db).await
    {
        let res = repo
            .add(&EnergyRecord {
                id: None,
                unit: "kWh".try_into().unwrap(),
                sub_system: "Battery".try_into().unwrap(),
                energy_source: "Solar".try_into().unwrap(),
                label: "hoge".to_string(),
                value: 1.0,
                monitored_at: chrono::Local::now(),
            })
            .await;

        match res {
            Ok(_) => "Successful.",
            Err(_) => "DB error.",
        }
    } else {
        "Environment variables for DB connection are not set."
    }
}
