use dotenvy::var;
use sea_orm::{ActiveValue, entity::*};

use crate::get_connection;
use crate::models::prelude::Units;
use crate::models::units;

pub async fn add_kwh() -> anyhow::Result<()> {
    let user = var("DB_OPERATOR_NAME")?;
    let password = var("DB_OPERATOR_PASSWORD")?;

    let kwh = units::ActiveModel {
        unit: ActiveValue::Set("kWh".to_owned()),
        ..Default::default()
    };
    let db = get_connection(&user, &password).await?;
    let res = Units::insert(kwh).exec(&db).await?;
    println!("Inserted unit with ID: {}", res.last_insert_id);
    Ok(())
}
