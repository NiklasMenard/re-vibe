mod common;

use application::user::{create::register_user, login::check_email_password};
use rocket::http::Status;
use rocket::serde::json::Json;
use serial_test::serial;
use shared::request_models::Credentials;

#[tokio::test]
#[serial]
async fn test_register_user_success() {
    let pool = common::setup_test_pool().await;
    let test_email = "newuser@test.com";

    // Cleanup first in case of previous test failure
    common::cleanup_test_user(&pool, test_email).await;

    let credentials = Json(Credentials {
        email: test_email.to_string(),
        password: "securepassword123".to_string(),
    });

    let result = register_user(&pool, credentials).await;

    assert_eq!(result, Status::Accepted, "User registration should succeed");

    // Cleanup
    common::cleanup_test_user(&pool, test_email).await;
}

#[tokio::test]
#[serial]
async fn test_register_duplicate_user_returns_conflict() {
    let pool = common::setup_test_pool().await;
    let test_email = "duplicate@test.com";

    // Cleanup first in case of previous test failure
    common::cleanup_test_user(&pool, test_email).await;

    let credentials1 = Json(Credentials {
        email: test_email.to_string(),
        password: "password123".to_string(),
    });

    let credentials2 = Json(Credentials {
        email: test_email.to_string(),
        password: "password123".to_string(),
    });

    // First registration should succeed
    let result1 = register_user(&pool, credentials1).await;
    assert_eq!(result1, Status::Accepted);

    // Second registration should fail with conflict
    let result2 = register_user(&pool, credentials2).await;
    assert_eq!(
        result2,
        Status::Conflict,
        "Duplicate email should return Conflict"
    );

    // Cleanup
    common::cleanup_test_user(&pool, test_email).await;
}

#[tokio::test]
#[serial]
async fn test_check_email_password_valid_credentials() {
    let pool = common::setup_test_pool().await;
    let test_email = "validuser@test.com";
    let test_password = "correctpassword";

    // Cleanup first in case of previous test failure
    common::cleanup_test_user(&pool, test_email).await;

    // Create test user
    common::create_test_user(&pool, test_email, test_password).await;

    // Test login with correct credentials
    let result =
        check_email_password(&pool, test_email.to_string(), test_password.to_string()).await;

    assert!(
        result.is_some(),
        "Valid credentials should return user role"
    );

    let user_role = result.unwrap();
    assert_eq!(user_role.role_id, 1, "Default role should be 1");

    // Cleanup
    common::cleanup_test_user(&pool, test_email).await;
}

#[tokio::test]
#[serial]
async fn test_check_email_password_invalid_password() {
    let pool = common::setup_test_pool().await;
    let test_email = "testuser@test.com";
    let test_password = "correctpassword";

    // Create test user
    common::create_test_user(&pool, test_email, test_password).await;

    // Test login with wrong password
    let result =
        check_email_password(&pool, test_email.to_string(), "wrongpassword".to_string()).await;

    assert!(result.is_none(), "Invalid password should return None");

    // Cleanup
    common::cleanup_test_user(&pool, test_email).await;
}

#[tokio::test]
#[serial]
async fn test_check_email_password_nonexistent_user() {
    let pool = common::setup_test_pool().await;

    let result = check_email_password(
        &pool,
        "nonexistent@test.com".to_string(),
        "anypassword".to_string(),
    )
    .await;

    assert!(result.is_none(), "Non-existent user should return None");
}

#[tokio::test]
#[serial]
async fn test_register_user_with_empty_password() {
    let pool = common::setup_test_pool().await;
    let test_email = "emptypass@test.com";

    let credentials = Json(Credentials {
        email: test_email.to_string(),
        password: "".to_string(),
    });

    let result = register_user(&pool, credentials).await;

    // Should still create user (password validation could be added later)
    assert_eq!(result, Status::Accepted);

    // Cleanup
    common::cleanup_test_user(&pool, test_email).await;
}
