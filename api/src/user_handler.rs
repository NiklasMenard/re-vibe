use domain::models::User;
use infrastructure::auth::ApiKey;

use rocket::response::status::{Created, NotFound};
use rocket::serde::json::{json, Json};
use rocket::{delete, get, post, put};

use serde_json::Value;

use shared::request_models::Credentials;
use shared::response_models::{Response, ResponseBody};

use uuid::Uuid;

use application::user::{create, delete, read, update};

#[get("/info")]
pub fn info(key: ApiKey) -> Json<Value> {
    Json(json!({
        "success":true,
        "message": key.0
    }
    ))
}

#[get("/<id>")]
pub fn find_user_by_id(id: String) -> Result<String, NotFound<String>> {
    let user = read::user_information(Uuid::parse_str(&id).unwrap())?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<id>", data = "<user>")]
pub fn update_user(id: String, user: Json<User>) -> Result<String, NotFound<String>> {
    let user = update::update_post(id, user)?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/<id>")]
pub fn delete_user(id: String) -> Json<Value> {
    Json(json!({
        "success": delete::delete(Uuid::parse_str(&id).unwrap())
    }))
}

#[get("/sensitive")]
pub fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn register(credentials: Json<Credentials>) -> Created<String> {
    create::register_user(credentials)
}
