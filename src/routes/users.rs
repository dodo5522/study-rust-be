use axum::{extract::Extension, Json};
use sqlx::PgPool;
use crate::models::user::User;
use uuid::Uuid;

pub async fn list_users(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)> {
    // 単純なクエリ例
    let rows = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY name LIMIT 100")
        .fetch_all(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows))
}