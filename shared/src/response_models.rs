// shared/src/response_models.rs

use domain::models::{Post, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Post(Post),
    Posts(Vec<Post>),
    User(User),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

pub struct LoginResponse {
    pub success: String,
    pub token: String,
}
