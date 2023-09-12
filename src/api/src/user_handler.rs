use domain::models::User;
use infrastructure::auth::ApiKey;

use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use shared::request_models::Credentials;
use shared::response_models::{Response, ResponseBody};

use uuid::Uuid;

use application::user::{create, delete, read, update};

#[get("/<id>")]
pub fn list_user_handler(key: ApiKey, id: String) -> Result<String, Status> {
    match ApiKey::verify_admin_role(&key.role) {
        true => {
            let user = read::get_by_id(Uuid::parse_str(&id).unwrap()).unwrap();

            let response = Response {
                body: ResponseBody::User(user),
            };
            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[put("/<id>", data = "<user>")]
pub fn update_user_handler(key: ApiKey, id: String, user: Json<User>) -> Result<String, Status> {
    match ApiKey::verify_admin_role(&key.role) {
        true => {
            let user = update::update_user(id, user).unwrap();

            let response = Response {
                body: ResponseBody::User(user),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[delete("/<id>")]
pub fn delete_user_handler(key: ApiKey, id: String) -> Status {
    match ApiKey::verify_admin_role(&key.role) {
        true => {
            delete::delete_user(Uuid::parse_str(&id).unwrap()).unwrap();
            Status::NoContent
        }
        _ => Status::Unauthorized,
    }
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub fn register_user_handler(credentials: Json<Credentials>) -> Status {
    create::register_user(credentials)
}
