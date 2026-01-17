use chrono::{Duration, TimeZone, Utc};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::Responder;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::env;

#[derive(PartialEq, Clone)]
pub enum UserRole {
    User,
    Seller,
    Admin,
}

#[derive(Clone)]
pub struct ApiKey {
    pub key: String,
    pub role: UserRole,
    pub exp: String,
}

impl ApiKey {
    fn from_claims(claims: &BTreeMap<String, String>) -> Option<Self> {
        let role_str = claims.get("role").map(|role| role.as_str());

        let role = match role_str {
            Some("1") => UserRole::User,
            Some("2") => UserRole::Seller,
            Some("3") => UserRole::Admin,
            _ => return None,
        };

        Some(ApiKey {
            key: claims.get("sub")?.to_string(),
            role,
            exp: claims.get("exp")?.to_string(),
        })
    }
}

#[derive(Clone)]
pub struct UserApiKey(pub ApiKey);

#[derive(Clone)]
pub struct AdminApiKey(pub ApiKey);

pub trait RoleVerifier {
    fn verify_role(&self) -> Outcome<Self, (Status, NetworkResponse), Status>
    where
        Self: Sized;
}

impl RoleVerifier for UserApiKey {
    fn verify_role(&self) -> Outcome<Self, (Status, NetworkResponse), Status> {
        if matches!(
            self.0.role,
            UserRole::User | UserRole::Seller | UserRole::Admin
        ) {
            Outcome::Success(self.clone())
        } else {
            Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Unauthorized".to_string()),
            ))
        }
    }
}

impl RoleVerifier for AdminApiKey {
    fn verify_role(&self) -> Outcome<Self, (Status, NetworkResponse), Status> {
        if matches!(self.0.role, UserRole::Admin) {
            Outcome::Success(self.clone())
        } else {
            Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Unauthorized".to_string()),
            ))
        }
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
        _ => false,
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
        match ApiKey::from_claims(claims) {
            Some(api_key) => Ok(api_key),
            None => Err(TokenReadError::ParsingFailure(
                "Invalid or missing claims".to_string(),
            )),
        }
    } else {
        Err(TokenReadError::ParsingFailure(
            "Error with algorithm type".to_string(),
        ))
    }
}

async fn extract_token_from_request(
    request: &Request<'_>,
) -> Outcome<ApiKey, (Status, NetworkResponse), Status> {
    let keys: Vec<_> = request.headers().get("Authorization").collect();

    match keys.len() {
        0 => Outcome::Error((
            Status::Unauthorized,
            NetworkResponse::Unauthorized("Unauthorized".to_string()),
        )),
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        extract_token_from_request(request).await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserApiKey {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match extract_token_from_request(request).await {
            Outcome::Success(api_key) => {
                let user_api_key = UserApiKey(api_key);
                user_api_key.verify_role()
            }
            Outcome::Error((status, response)) => Outcome::Error((status, response)),
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminApiKey {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match extract_token_from_request(request).await {
            Outcome::Success(api_key) => {
                let admin_api_key = AdminApiKey(api_key);
                admin_api_key.verify_role()
            }
            Outcome::Error((status, response)) => Outcome::Error((status, response)),
            Outcome::Forward(status) => Outcome::Forward(status),
        }
    }
}
