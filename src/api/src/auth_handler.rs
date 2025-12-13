use chrono::Utc;
use infrastructure::database::connection::DbPool;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::post;
use rocket::serde::json::{json, Json};
use rocket::State;
use serde_json::Value;

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha384;

use application::user::login;
use shared::request_models::Credentials;
use std::collections::BTreeMap;
use std::env;

// Function to create a new Header with the desired configuration
fn create_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    }
}

// Function to generate access and refresh tokens
fn generate_tokens(
    user_id: &str,
    role_id: &str,
    access_duration_sec: i64,
    refresh_duration_min: i64,
) -> Result<(String, String), Status> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    // Generate Access Token
    let access_expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(access_duration_sec))
        .expect("Invalid timestamp")
        .timestamp();

    let mut access_claims = BTreeMap::new();
    access_claims.insert("sub".to_string(), user_id.to_string());
    access_claims.insert("role".to_string(), role_id.to_string());
    access_claims.insert("exp".to_string(), access_expiration.to_string());

    let access_header = create_header();
    let access_token = Token::new(access_header, access_claims)
        .sign_with_key(&key)
        .map_err(|_| Status::InternalServerError)?;

    // Generate Refresh Token
    let refresh_expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(refresh_duration_min))
        .expect("Invalid timestamp")
        .timestamp();

    let mut refresh_claims = BTreeMap::new();
    refresh_claims.insert("sub".to_string(), user_id.to_string());
    refresh_claims.insert("role".to_string(), role_id.to_string());
    refresh_claims.insert("exp".to_string(), refresh_expiration.to_string());

    let refresh_header = create_header();
    let refresh_token = Token::new(refresh_header, refresh_claims)
        .sign_with_key(&key)
        .map_err(|_| Status::InternalServerError)?;

    Ok((
        access_token.as_str().to_string(),
        refresh_token.as_str().to_string(),
    ))
}

fn generate_refresh_cookie(token: String) -> Cookie<'static> {
    let mut cookie = Cookie::new("refresh_token", token);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(true);
    cookie.set_path("/");

    cookie
}

#[post("/login", data = "<credentials>")]
pub async fn login_handler(
    pool: &State<DbPool>,
    credentials: Json<Credentials>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Value>, Status> {
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();

    match login::check_email_password(pool.inner(), email, password).await {
        None => Err(Status::Unauthorized),
        Some(user) => {
            let (access_token, refresh_token) = generate_tokens(
                &user.user_id.to_string(),
                &user.role_id.to_string(),
                300,
                10,
            )?;

            // Set the refresh token in an HTTP-only cookie
            let cookie = generate_refresh_cookie(refresh_token);
            cookies.add(cookie);

            // Return the access token in the response body
            Ok(Json(json!({
                "success": true,
                "access_token": access_token
            })))
        }
    }
}

#[post("/refresh")]
pub fn refresh_token_handler(cookies: &CookieJar<'_>) -> Result<Json<Value>, Status> {
    // Retrieve the refresh token from the cookies
    let refresh_token_cookie = cookies.get("refresh_token");

    // Check if the refresh token cookie exists
    if let Some(refresh_token) = refresh_token_cookie {
        // Decode and verify the token
        let token =
            Token::<Header, BTreeMap<String, String>, _>::parse_unverified(refresh_token.value())
                .map_err(|_| Status::Unauthorized)?;

        // Access the claims from the token
        let claims: &BTreeMap<String, String> = token.claims();

        // Validate expiration or other claims if needed
        let exp = claims
            .get("exp")
            .and_then(|exp_str| exp_str.parse::<i64>().ok())
            .unwrap_or(0);

        if exp < Utc::now().timestamp() {
            return Err(Status::Unauthorized); // Token expired
        }

        // Extract user ID and role ID
        let user_id = claims.get("sub").ok_or(Status::Unauthorized)?;
        let role_id = claims.get("role").ok_or(Status::Unauthorized)?;

        // Issue new tokens
        let (new_access_token, new_refresh_token) = generate_tokens(user_id, role_id, 300, 10)?;

        // Create a new refresh token cookie

        let new_cookie = generate_refresh_cookie(new_refresh_token.clone());

        // Add the new cookie to the jar
        cookies.add(new_cookie);

        // Return the new access token and refresh token in the response body
        Ok(Json(json!({
            "success": true,
            "access_token": new_access_token.clone(),
            "refresh_token": new_refresh_token
        })))
    } else {
        Err(Status::Unauthorized) // No refresh token in cookies
    }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
    let mut cookie = Cookie::new("refresh_token", "");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(true);
    cookie.set_path("/");

    cookies.remove(cookie);
}
