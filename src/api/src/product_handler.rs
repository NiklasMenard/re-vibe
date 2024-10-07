use application::product::{create, delete, read, update};
use domain::models::NewProduct;
use infrastructure::auth::UserApiKey;
use rocket::http::Status;
use shared::request_models::ProductFilter;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[get(
    "/products?<page>&<page_size>",
    format = "application/json",
    data = "<filter>"
)]
pub async fn list_products_handler(
    page: Option<i64>,
    page_size: Option<i64>,
    filter: Option<Json<ProductFilter>>,
) -> Result<String, Status> {
    read::list_products(page, page_size, filter).await
}

#[get("/products/<product_id>")]
pub async fn list_product_handler(product_id: i32) -> Result<String, Status> {
    read::list_product(product_id).await
}

#[put(
    "/products/<product_id>",
    format = "application/json",
    data = "<product>"
)]
pub async fn update_product_handler(
    _key: UserApiKey,
    product_id: i32,
    product: Json<NewProduct>,
) -> Result<String, Status> {
    update::update_product(product_id, product).await
}

#[post("/products", format = "application/json", data = "<product>")]
pub async fn create_product_handler(
    _key: UserApiKey,
    product: Json<NewProduct>,
) -> Result<String, Status> {
    create::post_product(product).await
}

#[delete("/products/<id>")]
pub async fn delete_product_handler(_key: UserApiKey, id: i32) -> Result<String, Status> {
    delete::delete_product(id).await
}
