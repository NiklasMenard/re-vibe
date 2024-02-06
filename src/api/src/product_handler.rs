use application::product::{create, delete, read, update};
use domain::models::NewProduct;
use infrastructure::auth::ApiKey;

use rocket::http::Status;
use rocket::response::status::Created;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::response_models::{Response, ResponseBody};

#[get("/products")]
pub fn list_products_handler(key: ApiKey) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let products = read::list_products();

            let response = Response {
                body: ResponseBody::Products(products),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[get("/products/<product_id>")]
pub fn list_product_handler(key: ApiKey, product_id: i32) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let found_product = read::list_product(product_id).unwrap();

            let response = Response {
                body: ResponseBody::Product(found_product),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[put(
    "/products/<product_id>",
    format = "application/json",
    data = "<product>"
)]
pub fn update_product_handler(
    key: ApiKey,
    product_id: i32,
    product: Json<NewProduct>,
) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let product = update::update_product(product_id, product).unwrap();

            let response = Response {
                body: ResponseBody::Product(product),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}

#[post("/products", format = "application/json", data = "<product>")]
pub fn create_product_handler(
    key: ApiKey,
    product: Json<NewProduct>,
) -> Result<Created<String>, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => Ok(create::post_product(product)),
        _ => return Err(Status::Unauthorized),
    }
}

#[delete("/products/<id>")]
pub fn delete_product_handler(key: ApiKey, id: i32) -> Result<String, Status> {
    match ApiKey::verify_user_role(&key.role) {
        true => {
            let products = delete::delete_product(id).unwrap();

            let response = Response {
                body: ResponseBody::Products(products),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }
        _ => return Err(Status::Unauthorized),
    }
}
