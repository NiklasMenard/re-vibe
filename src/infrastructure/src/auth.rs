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

#[derive(PartialEq)]
pub enum UserRole {
    User,
    Seller,
    Admin,
}

pub struct ApiKey {
    pub key: String,
    pub role: UserRole,
    pub exp: String,
}

impl ApiKey {
    fn from_claims(claims: &BTreeMap<String, String>) -> Option<Self> {
        // Parse the role from claims
        let role_str = claims.get("role").and_then(|role| Some(role.as_str()));

        // Convert the role string to UserRole enum variant
        let role = match role_str {
            Some("1") => UserRole::User,
            Some("2") => UserRole::Seller,
            Some("3") => UserRole::Admin,
            _ => return None, // Handle invalid role strings
        };

        Some(ApiKey {
            key: claims.get("sub")?.to_string(),
            role,
            exp: claims.get("exp")?.to_string(),
        })
    }
}

pub struct UserApiKey(pub ApiKey);

impl UserApiKey {
    fn verify_role(role: &UserRole) -> bool {
        *role == UserRole::User || *role == UserRole::Seller || *role == UserRole::Admin
    }
}

pub struct AdminApiKey(pub ApiKey);

impl AdminApiKey {
    fn verify_role(role: &UserRole) -> bool {
        *role == UserRole::Admin
    }
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

fn timestamp_expired(timestamp: i64) -> bool {
    let seconds = 600;

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
    let token_str = Token::parse_unverified(&incoming[7..])
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

    if header.algorithm == AlgorithmType::Hs384 {
        let new_api_key = match ApiKey::from_claims(&claims) {
            Some(api_key) => Ok(api_key),
            None => Err(TokenReadError::ParsingFailure(
                "Invalid or missing claims".to_string(),
            )),
        };
        new_api_key
    } else {
        Err(TokenReadError::ParsingFailure(
            "Error with algorithm type".to_string(),
        ))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        match keys.len() {
            0 => {
                return Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized("Unauthorized".to_string()),
                ));
            }
            1 => match read_token(keys[0]) {
                Ok(api_key) => Outcome::Success(api_key),
                Err(err) => match err {
                    TokenReadError::Expired => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::BadRequest("Token Expired".to_string()),
                    )),

                    TokenReadError::ParsingFailure(msg) => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::BadRequest(msg.to_string()),
                    )),
                    _ => Outcome::Error((
                        Status::BadRequest,
                        NetworkResponse::BadRequest("Bad request".to_string()),
                    )),
                },
            },
            _ => Outcome::Error((
                Status::BadRequest,
                NetworkResponse::BadRequest("Bad request".to_string()),
            )),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserApiKey {
    type Error = NetworkResponse;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<UserApiKey, NetworkResponse> {
        match ApiKey::from_request(request).await {
            Outcome::Success(api_key) => {
                if !UserApiKey::verify_role(&api_key.role) {
                    return Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized("Unauthorized".to_string()),
                    ));
                }
                Outcome::Success(UserApiKey(api_key))
            }
            Outcome::Error((status, network_response)) => {
                Outcome::Error((status, network_response))
            }
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminApiKey {
    type Error = NetworkResponse;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<AdminApiKey, NetworkResponse> {
        match ApiKey::from_request(request).await {
            Outcome::Success(api_key) => {
                if !AdminApiKey::verify_role(&api_key.role) {
                    return Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized("Unauthorized".to_string()),
                    ));
                }
                Outcome::Success(AdminApiKey(api_key))
            }
            Outcome::Error((status, network_response)) => {
                Outcome::Error((status, network_response))
            }
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}
