use axum::{extract::State, http::{header, StatusCode}, response::Response, Error};
use serde_json::json;
use sqlx::PgPool;

use crate::{models::user::User, security::jwt::Auth};
pub async fn get_curr_user(State(state): State<PgPool>, user_id: Auth) -> Response<String> {
    let db_pool = &state;
    
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(&user_id.id)
    .fetch_one(db_pool)
    .await
    .unwrap();

    Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(
      json!({
        "data": {
            "full_name": user.full_name
        }
      })
      .to_string(),
    )
    .unwrap_or_default()
}