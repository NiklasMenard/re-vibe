use diesel::result::Error;
use domain::models::User;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use hmac::{Hmac, NewMac};
use jwt::{AlgorithmType, Header, Token};
use sha2::Sha384;
use std::collections::BTreeMap;

use std::time::{SystemTime, UNIX_EPOCH};
// use crypto::sha2::Sha256;
use uuid::Uuid;

use application::user::{create, delete, read, update};

#[post("/register", format = "application/json", data = "<credentials>")]
fn register(credentials: Json<User>) -> Result<status::Created<Json<User>>, Status> {
    create::register_user(insert);
}

#[get("/info")]
fn info(key: ApiKey) -> Json<JsonValue> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

#[get("/info", rank = 2)]
fn info_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[get("/<id>")]
fn read_one(
    _key: ApiKey,
    id: String,
    connection: db::Connection,
) -> Result<Json<JsonValue>, Status> {
    User::read(Uuid::parse_str(&id).unwrap(), &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[put("/<id>", data = "<user>")]
fn update(id: String, user: Json<User>, connection: db::Connection) -> Json<JsonValue> {
    let update = User {
        ..user.into_inner()
    };
    Json(json!({
        "success": User::update(Uuid::parse_str(&id).unwrap(), update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: String, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(Uuid::parse_str(&id).unwrap(), &connection)
    }))
}

#[get("/sensitive")]
fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    email: String,
    password: String,
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>) -> Result<Json<JsonValue>, Status> {
    let email = credentials.email.to_string();
    let password = credentials.password.to_string();
    // Expiration of the token is set to two weeks
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let two_weeks_from_now: u64 = since_the_epoch.as_secs() + 1209600 as u64;

    match by_email_and_password(email, password) {
        None => Err(Status::NotFound),
        Some(user) => {
            let key: Hmac<Sha384> = Hmac::new_varkey(b"some-secret").unwrap();
            let header = Header {
                algorithm: AlgorithmType::Hs384,
                ..Default::default()
            };

            let mut claims = BTreeMap::new();
            claims.insert("sub", Some(user.id.to_hyphenated().to_string()));

            let token = Token::new(header, claims);

            token
                .sign_with_key(&key)
                .map(|message| Json(json!({ "success": true, "token": message.as_str() })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

fn person_created(user: User) -> status::Created<Json<User>> {
    status::Created(
        format!(
            "{host}:{port}/user/{name}",
            host = host(),
            port = port(),
            name = user.id
        )
        .to_string(),
        Some(Json(user)),
    )
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
