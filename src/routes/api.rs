
use axum::{routing::{delete, get, post, put}, Router};
use crate::{services::{auth, book::{create_book, delete_book_by_id, get_all_books, get_book_by_id, update_book_by_id}, user}, AppState};

pub fn router(app_state: &AppState) -> Router {

    return Router::new()
    .route("/login", post(auth::login))
    .route("/register", post(auth::register))
    .route("/me", get(user::get_curr_user))
    .route("/book", post(create_book))
    .route("/book", get(get_all_books))
    .route("/book/{id}", get(get_book_by_id))
    .route("/book/{id}", delete(delete_book_by_id))
    .route("/book/{id}", put(update_book_by_id))
    .with_state(app_state.db_pool.clone());
}