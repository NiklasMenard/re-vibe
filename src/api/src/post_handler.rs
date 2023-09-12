use application::post::{create, delete, read, update};
use domain::models::NewPost;
use infrastructure::auth::ApiKey;

use rocket::http::Status;
use rocket::response::status::Created;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::response_models::{Response, ResponseBody};

#[get("/posts")]
pub fn list_posts_handler(key: ApiKey) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let posts = read::list_posts();

            let response = Response {
                body: ResponseBody::Posts(posts),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[get("/posts/<post_id>")]
pub fn list_post_handler(key: ApiKey, post_id: i32) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let found_post = read::list_post(post_id).unwrap();

            let response = Response {
                body: ResponseBody::Post(found_post),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[put("/posts/<post_id>", format = "application/json", data = "<post>")]
pub fn update_post_handler(
    key: ApiKey,
    post_id: i32,
    post: Json<NewPost>,
) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let post = update::update_post(post_id, post).unwrap();

            let response = Response {
                body: ResponseBody::Post(post),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[post("/posts", format = "application/json", data = "<post>")]
pub fn create_post_handler(key: ApiKey, post: Json<NewPost>) -> Result<Created<String>, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => Ok(create::create_post(post)),
        _ => return Err(Status::Unauthorized),
    }
}

#[delete("/posts/<id>")]
pub fn delete_post_handler(key: ApiKey, id: i32) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let posts = delete::delete_post(id).unwrap();

            let response = Response {
                body: ResponseBody::Posts(posts),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}
