use infrastructure::validation::{Validi32, ValidUuid};
use rocket::request::FromParam;
use rocket::http::Status;
use uuid::Uuid;

#[test]
fn test_valid_uuid_parsing() {
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let result = ValidUuid::from_param(uuid_str);

    assert!(result.is_ok(), "Valid UUID should parse successfully");
    assert_eq!(result.unwrap().value.to_string(), uuid_str);
}

#[test]
fn test_invalid_uuid_parsing() {
    let invalid_uuid = "not-a-valid-uuid";
    let result = ValidUuid::from_param(invalid_uuid);

    assert!(result.is_err(), "Invalid UUID should return error");
    assert_eq!(result.unwrap_err(), Status::BadRequest);
}

#[test]
fn test_malformed_uuid_parsing() {
    let malformed_uuid = "550e8400-e29b-41d4-a716";
    let result = ValidUuid::from_param(malformed_uuid);

    assert!(result.is_err(), "Malformed UUID should return error");
}

#[test]
fn test_valid_i32_parsing() {
    let result = Validi32::from_param("42");

    assert!(result.is_ok(), "Valid i32 should parse successfully");
    assert_eq!(result.unwrap().value, 42);
}

#[test]
fn test_negative_i32_parsing() {
    let result = Validi32::from_param("-123");

    assert!(result.is_ok(), "Negative i32 should parse successfully");
    assert_eq!(result.unwrap().value, -123);
}

#[test]
fn test_invalid_i32_parsing() {
    let result = Validi32::from_param("not-a-number");

    assert!(result.is_err(), "Invalid i32 should return error");
    assert_eq!(result.unwrap_err(), Status::BadRequest);
}

#[test]
fn test_i32_overflow_parsing() {
    let overflow = "9999999999999999999";
    let result = Validi32::from_param(overflow);

    assert!(result.is_err(), "Overflow value should return error");
}

#[test]
fn test_verify_user_id_matching() {
    use infrastructure::validation::verify_user_id;
    use infrastructure::auth::{UserApiKey, ApiKey, UserRole};

    let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let api_key = UserApiKey(ApiKey {
        key: uuid.to_string(),
        role: UserRole::User,
        exp: "9999999999".to_string(),
    });
    let valid_uuid = ValidUuid { value: uuid };

    let result = verify_user_id(&api_key, &valid_uuid);

    assert!(result.is_ok(), "Matching user IDs should verify successfully");
}

#[test]
fn test_verify_user_id_mismatch() {
    use infrastructure::validation::verify_user_id;
    use infrastructure::auth::{UserApiKey, ApiKey, UserRole};

    let uuid1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let uuid2 = Uuid::parse_str("660e8400-e29b-41d4-a716-446655440001").unwrap();

    let api_key = UserApiKey(ApiKey {
        key: uuid1.to_string(),
        role: UserRole::User,
        exp: "9999999999".to_string(),
    });
    let valid_uuid = ValidUuid { value: uuid2 };

    let result = verify_user_id(&api_key, &valid_uuid);

    assert!(result.is_err(), "Mismatched user IDs should return error");
    assert_eq!(result.unwrap_err(), Status::Forbidden);
}
