use chrono::Utc;
use infrastructure::auth::{ApiKey, UserRole};
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::{json, Json};
use serde_json::Value;

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
// use crypto::sha2::Sha256;
use sha2::Sha384;

use application::user::login;
use shared::request_models::Credentials;
use std::collections::BTreeMap;
use std::env;

#[post("/login", data = "<credentials>")]
pub fn login_handler(credentials: Json<Credentials>) -> Result<Json<Value>, Status> {
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();

    match login::check_email_password(email, password) {
        None => Err(Status::Unauthorized),
        Some(user) => {
            let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

            let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };

            let expiration = Utc::now()
                .checked_add_signed(chrono::Duration::seconds(300))
                .expect("Invalid timestamp")
                .timestamp();

            let mut claims = BTreeMap::new();
            claims.insert("sub", Some(user.user_id.to_string()));
            claims.insert("role", Some(user.role_id.to_string()));
            claims.insert("exp", Some(expiration.to_string()));

            let token = Token::new(header, claims);

            token
                .sign_with_key(&key)
                .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

#[post("/refresh")]
pub fn refresh_token_handler(api_key: ApiKey) -> Result<Json<Value>, Status> {
    let new_exp = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(300))
        .expect("Valid timestamp")
        .timestamp();

    // Replace the exp claim with the new expiration time
    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), api_key.key);
    claims.insert(
        "role".to_string(),
        match api_key.role {
            UserRole::User => "1".to_string(),
            UserRole::Seller => "2".to_string(),
            UserRole::Admin => "3".to_string(),
        },
    );
    claims.insert("exp".to_string(), new_exp.to_string());

    // Create a new token with the updated claims
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let token = Token::new(header, claims);

    token
        .sign_with_key(&key)
        .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
        .map_err(|_| Status::InternalServerError)
}
