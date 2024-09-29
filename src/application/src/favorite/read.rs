use diesel::prelude::*;
use domain::{
    models::Product,
    schema::{
        products, user_favorite_products,
        users::{self},
    },
};
use infrastructure::database::connection::establish_connection;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn list_favorite_products(user_id: Uuid) -> Result<String, Status> {
    let connect = &mut establish_connection();

    match user_favorite_products::table
        .inner_join(users::table.on(user_favorite_products::user_id.eq(users::id)))
        .inner_join(products::table.on(user_favorite_products::product_id.eq(products::product_id)))
        .filter(user_favorite_products::user_id.eq(user_id))
        .select((
            products::product_id,
            products::name,
            products::description,
            products::price,
            products::quantity,
            products::seller_id,
            products::category_id,
            products::creation_date,
            products::bucket_key,
        ))
        .load::<Product>(connect)
    {
        Ok(products) => {
            let response = Response {
                body: ResponseBody::Products(products),
            };
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            eprintln!("Database error - {}", err);
            Err(Status::InternalServerError)
        }
    }
}
