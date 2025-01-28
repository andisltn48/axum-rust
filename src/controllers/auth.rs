use axum::{routing::get, Router};

use crate::services::auth;

pub fn login() -> Router {
    return Router::new()
    .route("/login", get(auth::login));
}