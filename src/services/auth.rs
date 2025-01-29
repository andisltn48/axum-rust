

use axum::{body::Body, extract::State, http::{header, Response, StatusCode}, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use sqlx::{PgPool, Error};
use crate::models::user::{CreateUserRequest, LoginRequest, User};

pub async fn login(State(state): State<PgPool>, Json(request) : Json<LoginRequest>) -> Result<impl IntoResponse, impl IntoResponse> {

    let db_pool = &state;
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&request.username)
    .fetch_one(db_pool)
    .await;

    match user {
        Ok(user) => {
            if verify(&request.password, &user.password).unwrap() {
                let response = json!({
                    "data": user
                }).to_string();
            
                Ok(
                    Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(response).unwrap_or_default()
                )
            } else {
                let response = json!({
                    "errors": "Unauthorized"
                }).to_string();
                Err(
                    Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(response).unwrap_or_default()
                )
            }
        }
        Err(Error::RowNotFound) => {
            let response = json!({
                "errors": "Unauthorized"
            }).to_string();
            Err(
                Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(header::CONTENT_TYPE, "application/json")
                .body(response).unwrap_or_default()
            )
        }
        Err(e) => {
            let response = json!({
                "errors": "Internal server error"
            }).to_string();
            Err(
                Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(response).unwrap_or_default()
            )
        }
    }
}

pub async fn register(State(state): State<PgPool>, Json(request) : Json<CreateUserRequest>) -> Response<String> {

    let db_pool = &state;
    let hashed_password = hash(&request.password, DEFAULT_COST).unwrap(); // TODO: Measure time, cost)
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password, full_name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&request.username)
    .bind(hashed_password)
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