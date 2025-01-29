use axum::{routing::{get, post}, Router};

use crate::services::auth;

// pub fn router() -> Router {
//     return Router::new()
//     .route("/login", post(auth::login))
// }