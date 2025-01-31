use std::borrow::Cow;

use axum::{extract::State, http::{header, Response, StatusCode}, Json};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{models::book::{Book, CreateBookRequest}, security::jwt::Auth};

pub async fn create_book(_user_id: Auth, State(state): State<PgPool>, Json(request) : Json<CreateBookRequest>) -> Response<String> {

    if let Err(errors) = &request.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .into_iter()
            .map(|(field, errors)| {
                format!(
                    "{}: {}",
                    field,
                    errors
                        .iter()
                        .map(|e| e.message.as_ref().unwrap_or(&Cow::Owned(e.to_string())).to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            })
            .collect();

        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!({"errors": error_messages}).to_string()).unwrap_or_default();
    }

    let db_pool = &state;
    let book = sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, author, image_url) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&request.title)
    .bind(&request.author)
    .bind(&request.image_url)
    .fetch_one(db_pool)
    .await;

    match book {
        Ok(book) => {
            let response = json!({
                "data": {
                    "id": book.id,
                    "title": book.title,
                    "author": book.author,
                    "image_url": book.image_url
                }
            }).to_string();
            Response::builder().status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(response).unwrap_or_default()
        },
        Err(e) => {
            let response = json!({"errors": e.to_string()}).to_string();
            Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "application/json")
            .body(response).unwrap_or_default()
        }
        
    }
}