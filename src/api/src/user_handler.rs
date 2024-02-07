use domain::models::User;
use infrastructure::auth::AdminApiKey;

use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use shared::request_models::Credentials;
use shared::response_models::{Response, ResponseBody};

use uuid::Uuid;

use application::user::{create, delete, read, update};

#[get("/<id>")]
pub fn list_user_handler(_key: AdminApiKey, id: String) -> Result<String, Status> {
    let user = read::get_by_id(Uuid::parse_str(&id).unwrap()).unwrap();

    let response = Response {
        body: ResponseBody::User(user),
    };
    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<id>", data = "<user>")]
pub fn update_user_handler(
    _key: AdminApiKey,
    id: String,
    user: Json<User>,
) -> Result<String, Status> {
    let user = update::update_user(id, user).unwrap();

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/<id>")]
pub fn delete_user_handler(_key: AdminApiKey, id: String) -> Status {
    delete::delete_user(Uuid::parse_str(&id).unwrap()).unwrap();
    Status::NoContent
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn register_user_handler(credentials: Json<Credentials>) -> Status {
    create::register_user(credentials)
}
