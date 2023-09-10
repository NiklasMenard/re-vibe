use domain::models::User;
use infrastructure::auth::ApiKey;

use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::{json, Json};
use rocket::{delete, get, post, put};

use serde_json::Value;

use shared::request_models::Credentials;
use shared::response_models::{Response, ResponseBody};

use uuid::Uuid;

use application::user::{create, delete, read, update};

#[get("/<id>")]
pub fn list_user_handler(_key: ApiKey, id: String) -> Result<String, NotFound<String>> {
    let user = read::get_by_id(Uuid::parse_str(&id).unwrap())?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<id>", data = "<user>")]
pub fn update_user_handler(
    _key: ApiKey,
    id: String,
    user: Json<User>,
) -> Result<String, NotFound<String>> {
    let user = update::update_user(id, user)?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/<id>")]
pub fn delete_user_handler(_key: ApiKey, id: String) -> Json<Value> {
    Json(json!({
        "success": delete::delete_user(Uuid::parse_str(&id).unwrap())
    }))
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn register_user_handler(credentials: Json<Credentials>) -> Status {
    create::register_user(credentials)
}
