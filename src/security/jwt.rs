use std::env;

use serde::{Deserialize, Serialize};

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