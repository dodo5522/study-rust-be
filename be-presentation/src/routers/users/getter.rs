use axum::{Json, http::StatusCode};

use super::models::User;

pub async fn get() -> (StatusCode, Json<User>) {
  (StatusCode::OK, Json(User { id: 1234, username: "test".to_string()}))
}
