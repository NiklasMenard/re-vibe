use chrono::Utc;
use diesel_async::RunQueryDsl;
use domain::models::UserFavoriteProduct;
use infrastructure::database::connection::DbPool;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn favorite_product(pool: &DbPool, user_id: Uuid, product_id: i32) -> Result<String, Status> {
    use domain::schema::user_favorite_products;

    let mut connection = pool.get().await.map_err(|_| Status::InternalServerError)?;

    let new_favorite = UserFavoriteProduct {
        user_id,
        product_id,
        added_date: Utc::now().naive_utc(),
    };

    match diesel::insert_into(user_favorite_products::table)
        .values(&new_favorite)
        .get_result::<UserFavoriteProduct>(&mut connection)
        .await
    {
        Ok(_) => {
            let response = Response {
                body: ResponseBody::Message("Product favorited successfully".to_string()),
            };
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            eprintln!("Database error - {err}");
            Err(Status::InternalServerError)
        }
    }
}
