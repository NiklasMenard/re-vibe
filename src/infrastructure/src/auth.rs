use rocket::request::{self, FromRequest, Request};

use rocket::http::Status;

// use crypto::sha2::Sha256;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;

pub struct ApiKey(pub String);

pub fn read_token(incoming: &str) -> Result<String, String> {
    let token_str =
        Token::parse_unverified(incoming).map_err(|_| "Unable to parse key".to_string())?;

    let key: Hmac<Sha384> =
        Hmac::new_from_slice(b"some-secret").map_err(|_| "Unable to parse key".to_string())?;

    let token: Token<Header, BTreeMap<String, String>, _> = token_str
        .verify_with_key(&key)
        .map_err(|_| "Token not valid".to_string())?;
    let header = token.header();
    let claims = token.claims();

    if header.algorithm == AlgorithmType::Hs384 {
        Ok(claims["sub"].clone())
    } else {
        Err("Token not valid".to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = Option<String>;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, Option<String>> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();

        match keys.len() {
            0 => return request::Outcome::Failure((Status::Unauthorized, None)),
            1 => {
                return match read_token(keys[0]) {
                    Ok(claim) => request::Outcome::Success(ApiKey(claim)),
                    Err(error_message) => {
                        request::Outcome::Failure((Status::BadRequest, Some(error_message)))
                    }
                }
            }
            _ => return request::Outcome::Failure((Status::BadRequest, None)),
        };
    }
}
