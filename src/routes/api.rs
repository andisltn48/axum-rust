
use axum::{http::{self, HeaderValue}, routing::{delete, get, post, put}, Router};
use tower_http::cors::CorsLayer;
use crate::{services::{auth, book::{create_book, delete_book_by_id, get_all_books, get_book_by_id, update_book_by_id, upload_book_image}, user}, AppState};

pub fn router(app_state: &AppState) -> Router {

    let cors = CorsLayer::new()
    .allow_origin(HeaderValue::from_static("http://localhost:3000"))
    .allow_methods([http::Method::GET, http::Method::POST, http::Method::PUT, http::Method::DELETE])
    .allow_headers([http::header::AUTHORIZATION, http::header::ACCEPT])
    .allow_credentials(true);


    return Router::new()
    .route("/login", post(auth::login))
    .route("/register", post(auth::register))
    .route("/me", get(user::get_curr_user))
    .route("/book", post(create_book))
    .route("/book", get(get_all_books))
    .route("/book/{id}", get(get_book_by_id))
    .route("/book/{id}", delete(delete_book_by_id))
    .route("/book/{id}", put(update_book_by_id))
    .route("/book/{id}/upload_image", post(upload_book_image))
    .layer(cors)
    .with_state(app_state.db_pool.clone());
}