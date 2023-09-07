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
pub fn find_user_by_id(_key: ApiKey, id: String) -> Result<String, NotFound<String>> {
    let user = read::user_information(Uuid::parse_str(&id).unwrap())?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<id>", data = "<user>")]
pub fn update_user(_key: ApiKey, id: String, user: Json<User>) -> Result<String, NotFound<String>> {
    let user = update::update_post(id, user)?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/<id>")]
pub fn delete_user(_key: ApiKey, id: String) -> Json<Value> {
    Json(json!({
        "success": delete::delete(Uuid::parse_str(&id).unwrap())
    }))
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn register(credentials: Json<Credentials>) -> Status {
    create::register_user(credentials)
}
