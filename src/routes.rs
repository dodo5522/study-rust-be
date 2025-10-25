use poem::web::Json;
use poem::{handler, IntoResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryOrder, Set};
use serde::Deserialize;

use crate::entity::user;

#[handler]
pub async fn list_users(db: poem::Data<sea_orm::DatabaseConnection>) -> impl IntoResponse {
    match user::Entity::find().order_by_asc(user::Column::Id).all(db.as_ref()).await {
        Ok(list) => Json(list),
        Err(e) => {
            tracing::error!("DB error listing users: {}", e);
            poem::Response::builder()
                .status(500)
                .body(format!("DB error: {}", e))
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[handler]
pub async fn create_user(
    db: poem::Data<sea_orm::DatabaseConnection>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // build active model
    let active = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        ..Default::default()
    };

    match active.insert(db.as_ref()).await {
        Ok(model) => (poem::http::StatusCode::CREATED, Json(model)).into_response(),
        Err(e) => {
            tracing::error!("DB error creating user: {}", e);
            poem::Response::builder()
                .status(500)
                .body(format!("DB error: {}", e))
        }
    }
}