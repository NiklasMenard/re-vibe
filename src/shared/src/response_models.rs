// shared/src/response_models.rs

use domain::models::{PaginatedProducts, Product, User};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseBody {
    Message(String),
    Product(Product),
    Products(Vec<Product>),
    PaginatedProducts(PaginatedProducts),
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
