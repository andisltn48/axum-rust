use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub full_name: String
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub full_name: String
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>
}