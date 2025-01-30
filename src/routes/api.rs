
use axum::{routing::{get, post}, Router};
use crate::{services::{auth, user}, AppState};

pub fn router(app_state: &AppState) -> Router {

    return Router::new()
    .route("/login", post(auth::login))
    .route("/register", post(auth::register))
    .route("/me", get(user::get_curr_user))
    // .nest("/api", controllers::auth::router())
    .with_state(app_state.db_pool.clone());
}