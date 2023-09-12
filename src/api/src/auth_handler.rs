use chrono::Utc;
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
            claims.insert("sub", Some(user.id.to_string()));
            claims.insert("role", Some(user.role.to_string()));
            claims.insert("exp", Some(expiration.to_string()));

            let token = Token::new(header, claims);

            token
                .sign_with_key(&key)
                .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}
