mod common;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rocket::http::{ContentType, Status};
use serial_test::serial;

async fn cleanup_test_user(email: &str) {
    use domain::schema::users;

    let pool = common::setup_test_pool().await;
    let mut conn = pool.get().await.unwrap();

    diesel::delete(users::table.filter(users::email.eq(email)))
        .execute(&mut conn)
        .await
        .ok();
}

async fn create_test_user(email: &str, password: &str) {
    use domain::models::{NewUser, NewUserRole};
    use domain::schema::{user_roles, users};

    let pool = common::setup_test_pool().await;
    let mut conn = pool.get().await.unwrap();

    let new_user = NewUser::from_credentials(email, password);

    // Insert user and get the created user to access the ID
    let user: domain::models::User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
        .await
        .expect("Failed to create test user");

    // Create user role
    let new_user_role = NewUserRole {
        user_id: user.id,
        role_id: 1,
    };

    diesel::insert_into(user_roles::table)
        .values(&new_user_role)
        .execute(&mut conn)
        .await
        .ok();
}

#[tokio::test]
#[serial]
async fn test_login_handler_success() {
    let test_email = "logintest@test.com";
    let test_password = "testpassword123";

    // Cleanup first in case of previous test failure
    cleanup_test_user(test_email).await;

    // Setup: create test user
    create_test_user(test_email, test_password).await;

    let client = common::setup_test_client().await;

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(format!(r#"{{"email": "{test_email}", "password": "{test_password}"}}"#))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok, "Login should succeed with valid credentials");

    let body = response.into_string().await.unwrap();
    assert!(body.contains("access_token"), "Response should contain access token");
    assert!(body.contains("\"success\":true"), "Response should indicate success");

    // Cleanup
    cleanup_test_user(test_email).await;
}

#[tokio::test]
#[serial]
async fn test_login_handler_invalid_credentials() {
    let test_email = "invalidlogin@test.com";

    let client = common::setup_test_client().await;

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(format!(r#"{{"email": "{test_email}", "password": "wrongpassword"}}"#))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized, "Login should fail with invalid credentials");
}

#[tokio::test]
#[serial]
async fn test_login_handler_malformed_json() {
    let client = common::setup_test_client().await;

    let response = client
        .post("/api/auth/login")
        .header(ContentType::JSON)
        .body(r#"{"email": "test@test.com"}"#) // Missing password field
        .dispatch()
        .await;

    assert_ne!(response.status(), Status::Ok, "Malformed JSON should not succeed");
}

#[tokio::test]
#[serial]
async fn test_create_user_success() {
    let test_email = "newuser@test.com";

    // Cleanup first in case of previous test failure
    cleanup_test_user(test_email).await;

    let client = common::setup_test_client().await;

    let response = client
        .post("/api/users/register")
        .header(ContentType::JSON)
        .body(format!(r#"{{"email": "{test_email}", "password": "newpassword123"}}"#))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Accepted, "User creation should succeed");

    // Cleanup
    cleanup_test_user(test_email).await;
}

#[tokio::test]
#[serial]
async fn test_create_duplicate_user() {
    let test_email = "duplicate@test.com";
    let test_password = "password123";

    // Cleanup first in case of previous test failure
    cleanup_test_user(test_email).await;

    // Create first user
    create_test_user(test_email, test_password).await;

    let client = common::setup_test_client().await;

    // Try to create duplicate
    let response = client
        .post("/api/users/register")
        .header(ContentType::JSON)
        .body(format!(r#"{{"email": "{test_email}", "password": "{test_password}"}}"#))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Conflict, "Duplicate user creation should return Conflict");

    // Cleanup
    cleanup_test_user(test_email).await;
}

#[tokio::test]
#[serial]
async fn test_logout_clears_cookie() {
    let client = common::setup_test_client().await;

    let response = client
        .post("/api/auth/logout")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok, "Logout should succeed");

    // Check that refresh_token cookie is cleared
    let cookies = response.cookies();
    let refresh_cookie = cookies.iter().find(|c| c.name() == "refresh_token");

    if let Some(cookie) = refresh_cookie {
        assert_eq!(cookie.value(), "", "Refresh token should be cleared");
    }
}
