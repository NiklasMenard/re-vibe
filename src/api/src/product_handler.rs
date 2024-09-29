use application::product::{create, delete, read, update};
use domain::models::NewProduct;
use infrastructure::auth::UserApiKey;

use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[get("/products")]
pub async fn list_products_handler() -> Result<String, Status> {
    read::list_products().await
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
