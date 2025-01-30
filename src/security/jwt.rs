use std::env;

use axum::{extract::{FromRequestParts, State}, http::{header, StatusCode}, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::{models::user::{self, User}, AppState};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub fn generate_token(user_id: String) -> String {
    let claims = Claims {
        sub: user_id,
        exp: chrono::Utc::now().timestamp() + 60 * 60
    };

    dotenv::dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|_| "Failed to generate token")
    .unwrap();

    return token;
}

pub struct Auth {
    pub id: i32
}

impl <S> FromRequestParts<S> for Auth 
where S: Send + Sync {
    type Rejection = Response<String>;

    async  fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self,Self::Rejection> {
        let token = parts.headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|str| str.split(" ").nth(1));
        match token {
            Some(token) => {
                let claims = decode_token(token);
                match claims {

                    Ok(claims) => {
                        Ok(Auth {
                            id: claims.sub.parse().unwrap()
                        })
                    },
                    Err(e) => {
                        println!("🔴 Error decoding token: {}", e);
                        let response = json!({"errors": e}).to_string();
                        Err(
                            Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(response).unwrap_or_default()
                        )
                    }
                }
            }
            None => {
                let response = json!({"errors": "No token provided"}).to_string();
                Err(
                    Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(response).unwrap_or_default()
                )
            }
        }
    }
}

fn decode_token(token: &str) -> Result<Claims, String> {
    dotenv::dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    );
    
    match token_data {
        Ok(token_data) => {
            Ok(token_data.claims)
        }
        Err(e) => Err(e.to_string()),
    }
}