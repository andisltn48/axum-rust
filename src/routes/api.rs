
use axum::{routing::{get, post}, Router};
use crate::{services::{auth, book::create_book, user}, AppState};

pub fn router(app_state: &AppState) -> Router {

    return Router::new()
    .route("/login", post(auth::login))
    .route("/register", post(auth::register))
    .route("/me", get(user::get_curr_user))
    .route("/book", post(create_book))
    .with_state(app_state.db_pool.clone());
}