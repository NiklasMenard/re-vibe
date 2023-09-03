use rocket::http::Status;
use rocket::post;
use rocket::serde::json::{json, Json};
use serde_json::Value;

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
// use crypto::sha2::Sha256;
use sha2::Sha384;

use application::user::create;
use shared::request_models::Credentials;
use std::collections::BTreeMap;

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>) -> Result<Json<Value>, Status> {
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();

    match create::register_user_by_email_and_password(email, password) {
        None => Err(Status::NotFound),
        Some(user) => {
            let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };

            let mut claims = BTreeMap::new();
            claims.insert("sub", Some(user.id.to_string()));

            let token = Token::new(header, claims);

            token
                .sign_with_key(&key)
                .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}
