use std::borrow::Cow;

use axum::{extract::{Multipart, Path, State}, http::{header, Response, StatusCode}, Json};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

use crate::{helper::file::upload_file, models::book::{Book, CreateBookRequest}, security::jwt::Auth};

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
        "INSERT INTO books (title, author) VALUES ($1, $2) RETURNING *"
    )
    .bind(&request.title)
    .bind(&request.author)
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

pub async fn get_all_books(_user_id: Auth, State(state): State<PgPool>) -> Response<String> {
    let db_pool = &state;
    let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
    .fetch_all(db_pool)
    .await;

    match books {
        Ok(books) => {
            let response = json!({"data": books}).to_string();
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

pub async fn get_book_by_id(_user_id: Auth, State(state): State<PgPool>, Path(book_id): Path<i32>) -> Response<String> {
    let db_pool = &state;
    let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1")
    .bind(book_id)
    .fetch_one(db_pool)
    .await;

    match book {
        Ok(book) => {
            let response = json!({"data": book}).to_string();
            Response::builder().status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(response).unwrap_or_default()
        },
        Err(sqlx::Error::RowNotFound) => {
            let response = json!({"errors": "Book not found"}).to_string();
            Response::builder().status(StatusCode::NOT_FOUND)
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

pub async fn delete_book_by_id(_user_id: Auth, State(state): State<PgPool>, Path(book_id): Path<i32>) -> Response<String> {
    let db_pool = &state;
    let book = sqlx::query_as::<_, Book>("DELETE FROM books WHERE id = $1 RETURNING *")
    .bind(book_id)
    .fetch_one(db_pool)
    .await;

    match book {
        Ok(_book) => {
            Response::builder().status(StatusCode::NO_CONTENT)
            .body("".to_string()).unwrap_or_default()
        },
        Err(sqlx::Error::RowNotFound) => {
            let response = json!({"errors": "Book not found"}).to_string();
            Response::builder().status(StatusCode::NOT_FOUND)
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

pub async fn update_book_by_id(_user_id: Auth, State(state): State<PgPool>, Path(book_id): Path<i32>, Json(request) : Json<CreateBookRequest>) -> Response<String> {
    let db_pool = &state;
    let book = sqlx::query_as::<_, Book>("UPDATE books SET title = $1, author = $2 WHERE id = $3 RETURNING *")
    .bind(&request.title)
    .bind(&request.author)
    .bind(book_id)
    .fetch_one(db_pool)
    .await;

    match book {
        Ok(book) => {
            let response = json!({"data": book}).to_string();
            Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(response).unwrap_or_default()
        },
        Err(sqlx::Error::RowNotFound) => {
            let response = json!({"errors": "Book not found"}).to_string();
            Response::builder().status(StatusCode::NOT_FOUND)
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

pub async fn upload_book_image(_user_id: Auth, State(state): State<PgPool>, Path(book_id): Path<i32>, multipart: Multipart) -> Response<String> {
    
    let images_url = upload_file(multipart).await;
    
    match images_url {
        Ok(images_url) => {
            let db_pool = &state;
            let book = sqlx::query_as::<_, Book>("UPDATE books SET image_url = $1 WHERE id = $2 RETURNING *")
            .bind(images_url)
            .bind(book_id)
            .fetch_one(db_pool)
            .await;
            let response = json!({"data": book.unwrap()}).to_string();
            Response::builder()
            .status(StatusCode::OK)
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