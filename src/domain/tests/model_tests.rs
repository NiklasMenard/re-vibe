use domain::models::{NewUser, User};

#[test]
fn test_new_user_from_credentials_creates_user() {
    let email = "test@example.com";
    let password = "password123";

    // NewUser::from_credentials should create a user without panicking
    let _user = NewUser::from_credentials(email, password);

    // If we get here, the user was created successfully
    // We can't test the internal fields since they're private,
    // but we can verify the function doesn't panic
}

#[test]
fn test_password_hashing_is_deterministic() {
    let salt = "dGVzdHNhbHQxMjM0NTY3OA=="; // base64 encoded test salt
    let password = "password123";

    let hash1 = User::hash_with_salt(password, salt);
    let hash2 = User::hash_with_salt(password, salt);

    assert_eq!(hash1, hash2, "Same password and salt should produce same hash");
}

#[test]
fn test_different_passwords_produce_different_hashes() {
    let salt = "dGVzdHNhbHQxMjM0NTY3OA==";

    let hash1 = User::hash_with_salt("password1", salt);
    let hash2 = User::hash_with_salt("password2", salt);

    assert_ne!(hash1, hash2, "Different passwords should produce different hashes");
}

#[test]
fn test_empty_password_hashing() {
    let salt = "dGVzdHNhbHQxMjM0NTY3OA==";

    let hash = User::hash_with_salt("", salt);

    assert!(!hash.is_empty(), "Empty password should still produce a hash");
}

#[test]
fn test_long_password_hashing() {
    let salt = "dGVzdHNhbHQxMjM0NTY3OA==";
    let long_password = "a".repeat(1000);

    let hash = User::hash_with_salt(&long_password, salt);

    assert!(!hash.is_empty(), "Long password should produce a hash");
}

#[test]
fn test_special_characters_in_password() {
    let salt = "dGVzdHNhbHQxMjM0NTY3OA==";
    let special_password = "p@ssw0rd!#$%^&*()";

    let hash = User::hash_with_salt(special_password, salt);

    assert!(!hash.is_empty(), "Password with special characters should hash successfully");
}
