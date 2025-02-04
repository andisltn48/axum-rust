use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, FromRow, Validate)]
pub struct Book {
    pub id: i32,

    pub title: String,

    pub author: String,

    pub image_url: String
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 255, message = "Author must be between 1 and 255 characters"))]
    pub author: String
}