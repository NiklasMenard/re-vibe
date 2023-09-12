use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::Responder;

use rocket::http::Status;

use hmac::{Hmac, Mac};

use chrono::{Duration, TimeZone, Utc};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};

use sha2::Sha384;
use std::collections::BTreeMap;
use std::env;

#[derive(Debug)]
pub struct ApiKey {
    pub key: String,
    pub role: String,
    pub exp: String,
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

pub enum TokenReadError {
    ParsingFailure(String),
    Unauthorized,
    Expired,
}

impl ApiKey {
    pub fn verify_user_role(role: &str) -> bool {
        return role == "user" || role == "admin";
    }

    pub fn verify_admin_role(role: &str) -> bool {
        return role == "admin";
    }
}

fn timestamp_expired(timestamp: i64) -> bool {
    let seconds = 300;

    match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(timestamp_datetime) => {
            let current_time = Utc::now();
            let time_difference = current_time - timestamp_datetime;
            time_difference <= Duration::seconds(seconds)
        }
        _ => false, // Handle the case where timestamp parsing fails
    }
}

pub fn read_token(incoming: &str) -> Result<ApiKey, TokenReadError> {
    let token_str = Token::parse_unverified(incoming)
        .map_err(|_| TokenReadError::ParsingFailure("Unable to parse key".to_string()))?;

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_secret.as_bytes())
        .map_err(|_| TokenReadError::ParsingFailure("Unable to parse key".to_string()))?;

    let token: Token<Header, BTreeMap<String, String>, _> = token_str
        .verify_with_key(&key)
        .map_err(|_| TokenReadError::ParsingFailure("Token not valid".to_string()))?;

    let header = token.header();
    let claims = token.claims();

    if !timestamp_expired(claims["exp"].parse::<i64>().unwrap()) {
        return Err(TokenReadError::Expired);
    }

    let new_api_key = ApiKey {
        key: claims["sub"].clone(),
        role: claims["role"].clone(),
        exp: claims["exp"].to_string(),
    };

    if header.algorithm == AlgorithmType::Hs384 {
        Ok(new_api_key)
    } else {
        Err(TokenReadError::ParsingFailure(
            "Error with algorithm type".to_string(),
        ))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, NetworkResponse> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();

        match keys.len() {
            0 => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized("Unauthorized".to_string()),
                ));
            }
            1 => {
                return match read_token(keys[0]) {
                    Ok(api_key) => request::Outcome::Success(api_key),
                    Err(err) => match err {
                        TokenReadError::Expired => {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                NetworkResponse::BadRequest("Token Expired".to_string()),
                            ));
                        }

                        TokenReadError::ParsingFailure(msg) => {
                            return Outcome::Failure((
                                Status::Unauthorized,
                                NetworkResponse::BadRequest(msg.to_string()),
                            ));
                        }
                        _ => {
                            return Outcome::Failure((
                                Status::BadRequest,
                                NetworkResponse::BadRequest("Bad request".to_string()),
                            ))
                        }
                    },
                }
            }
            _ => {
                return Outcome::Failure((
                    Status::BadRequest,
                    NetworkResponse::BadRequest("Bad request".to_string()),
                ))
            }
        };
    }
}
