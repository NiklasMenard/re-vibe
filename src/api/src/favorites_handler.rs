use application::favorite::{create, delete, read};
use infrastructure::auth::UserApiKey;

use rocket::http::Status;
use rocket::{delete, get, post};

use infrastructure::validation::{verify_user_id, ValidUuid, Validi32};

#[get("/<user_id>/favorites")]
pub async fn get_favorited_products(
    _key: UserApiKey,
    user_id: ValidUuid,
) -> Result<String, Status> {
    verify_user_id(&_key, &user_id)?;

    read::list_favorite_products(user_id.value).await
}

#[post("/<user_id>/favorites/<product_key>")]
pub async fn add_favorite_product(
    _key: UserApiKey,
    user_id: ValidUuid,
    product_key: Validi32,
) -> Result<String, Status> {
    verify_user_id(&_key, &user_id)?;

    create::favorite_product(user_id.value, product_key.value).await
}

#[delete("/<user_id>/favorites/<product_key>")]
pub async fn delete_product_handler(
    _key: UserApiKey,
    user_id: ValidUuid,
    product_key: Validi32,
) -> Result<String, Status> {
    verify_user_id(&_key, &user_id)?;

    delete::unfavorite_product(product_key.value).await
}
