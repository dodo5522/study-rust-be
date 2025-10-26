use axum::{http::StatusCode, Json};

use super::models::{CreateUser, User};

pub async fn create(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
