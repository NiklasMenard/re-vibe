use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;

fn create_test_key() -> Hmac<Sha384> {
    let secret = "test_jwt_secret_for_testing";
    Hmac::new_from_slice(secret.as_bytes()).unwrap()
}

fn create_header() -> Header {
    Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    }
}

#[test]
fn test_jwt_token_generation() {
    let key = create_test_key();
    let header = create_header();

    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), "user123".to_string());
    claims.insert("role".to_string(), "1".to_string());

    let token = Token::new(header, claims);
    let signed_token = token.sign_with_key(&key);

    assert!(signed_token.is_ok(), "Token should sign successfully");
}

#[test]
fn test_jwt_token_verification() {
    let key = create_test_key();
    let header = create_header();

    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), "user123".to_string());
    claims.insert("role".to_string(), "1".to_string());

    let token = Token::new(header, claims);
    let signed_token = token.sign_with_key(&key).unwrap();
    let token_string = signed_token.as_str();

    // Verify the token
    let verified: Result<Token<Header, BTreeMap<String, String>, _>, _> =
        token_string.verify_with_key(&key);

    assert!(verified.is_ok(), "Token should verify successfully");

    let verified_token = verified.unwrap();
    let verified_claims = verified_token.claims();

    assert_eq!(verified_claims.get("sub").unwrap(), "user123");
    assert_eq!(verified_claims.get("role").unwrap(), "1");
}

#[test]
fn test_jwt_token_with_expiration() {
    let key = create_test_key();
    let header = create_header();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .unwrap()
        .timestamp();

    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), "user123".to_string());
    claims.insert("exp".to_string(), expiration.to_string());

    let token = Token::new(header, claims);
    let signed_token = token.sign_with_key(&key).unwrap();

    // Verify token
    let verified: Result<Token<Header, BTreeMap<String, String>, _>, _> =
        signed_token.as_str().verify_with_key(&key);

    assert!(verified.is_ok());

    let verified_token = verified.unwrap();
    let verified_claims = verified_token.claims();
    let exp = verified_claims
        .get("exp")
        .and_then(|e| e.parse::<i64>().ok())
        .unwrap();

    assert!(exp > Utc::now().timestamp(), "Token should not be expired");
}

#[test]
fn test_expired_token_detection() {
    let key = create_test_key();
    let header = create_header();

    // Create an already expired token
    let expiration = Utc::now()
        .checked_sub_signed(chrono::Duration::seconds(3600))
        .unwrap()
        .timestamp();

    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), "user123".to_string());
    claims.insert("exp".to_string(), expiration.to_string());

    let token = Token::new(header, claims);
    let signed_token = token.sign_with_key(&key).unwrap();

    // Verify token (signature will be valid, but expiration check should fail)
    let verified: Result<Token<Header, BTreeMap<String, String>, _>, _> =
        signed_token.as_str().verify_with_key(&key);

    assert!(verified.is_ok(), "Token signature should be valid");

    let verified_token = verified.unwrap();
    let verified_claims = verified_token.claims();
    let exp = verified_claims
        .get("exp")
        .and_then(|e| e.parse::<i64>().ok())
        .unwrap();

    assert!(exp < Utc::now().timestamp(), "Token should be expired");
}

#[test]
fn test_token_with_invalid_signature() {
    let key1 = create_test_key();
    let key2 = Hmac::<Sha384>::new_from_slice(b"different_secret").unwrap();

    let header = create_header();
    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), "user123".to_string());

    let token = Token::new(header, claims);
    let signed_token = token.sign_with_key(&key1).unwrap();

    // Try to verify with different key
    let verified: Result<Token<Header, BTreeMap<String, String>, _>, _> =
        signed_token.as_str().verify_with_key(&key2);

    assert!(
        verified.is_err(),
        "Token should fail verification with wrong key"
    );
}
