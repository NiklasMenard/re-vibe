// shared/src/response_models.rs

use domain::models::{Product, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Product(Product),
    Products(Vec<Product>),
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
