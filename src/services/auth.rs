use std::sync::Arc;

use axum::{extract::State, http::{header, Response, StatusCode}, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;
use crate::{models::user::{CreateUserRequest, User}, AppState};

pub async fn login(Json(request) : Json<CreateUserRequest>) -> String {
    format!("Created user: {}, Email: {}", request.username, request.full_name)
}

pub async fn register(State(state): State<PgPool>, Json(request) : Json<CreateUserRequest>) -> Response<String> {

    let db_pool = &state;
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password, full_name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&request.username)
    .bind(&request.password)
    .bind(&request.full_name)
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(e.to_string()).unwrap_or_default();
    });

    let response = json!({
        "data": user.unwrap()
    }).to_string();

    // Ok((StatusCode::OK, response))
    Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "application/json")
    .body(response).unwrap_or_default()
}