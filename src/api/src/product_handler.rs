use application::product::{create, delete, read, update};
use domain::models::NewProduct;
use infrastructure::auth::UserApiKey;

use rocket::http::Status;
use rocket::response::status::Created;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::response_models::{Response, ResponseBody};

#[get("/products")]
pub async fn list_products_handler() -> Result<String, Status> {
    let products = read::list_products().await;

    let response = Response {
        body: ResponseBody::Products(products),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/products/<product_id>")]
pub fn list_product_handler(product_id: i32) -> Result<String, Status> {
    let found_product = read::list_product(product_id).unwrap();

    let response = Response {
        body: ResponseBody::Product(found_product),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put(
    "/products/<product_id>",
    format = "application/json",
    data = "<product>"
)]
pub fn update_product_handler(
    _key: UserApiKey,
    product_id: i32,
    product: Json<NewProduct>,
) -> Result<String, Status> {
    let product = update::update_product(product_id, product).unwrap();

    let response = Response {
        body: ResponseBody::Product(product),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/products", format = "application/json", data = "<product>")]
pub fn create_product_handler(
    _key: UserApiKey,
    product: Json<NewProduct>,
) -> Result<Created<String>, Status> {
    Ok(create::post_product(product))
}

#[delete("/products/<id>")]
pub fn delete_product_handler(_key: UserApiKey, id: i32) -> Result<String, Status> {
    let products = delete::delete_product(id).unwrap();

    let response = Response {
        body: ResponseBody::Products(products),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
