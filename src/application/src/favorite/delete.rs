use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::{models::UserFavoriteProduct, schema::user_favorite_products};
use infrastructure::database::connection::DbPool;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};

pub async fn unfavorite_product(pool: &DbPool, id: i32) -> Result<String, Status> {
    let mut connect = pool.get().await.map_err(|_| Status::InternalServerError)?;

    // Attempt to delete the product
    let num_deleted = match diesel::delete(
        user_favorite_products::table.filter(user_favorite_products::product_id.eq(id)),
    )
    .execute(&mut connect)
    .await
    {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err(Status::NotFound);
            }
            _ => {
                eprintln!("Database error - {err}");
                return Err(Status::InternalServerError);
            }
        },
    };

    // If product deletion was successful (at least one row deleted)
    if num_deleted > 0 {
        match user_favorite_products::table
            .load::<UserFavoriteProduct>(&mut connect)
            .await
        {
            Ok(products) => {
                if !products.is_empty() {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Successfully unfavorited product with {} ID.",
                            products[0].product_id
                        )),
                    };
                    Ok(serde_json::to_string(&response).unwrap())
                } else {
                    let response = Response {
                        body: ResponseBody::Message("No favorite products remaining.".to_string()),
                    };
                    Ok(serde_json::to_string(&response).unwrap())
                }
            }
            Err(err) => {
                eprintln!("Database error - {err}");
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::NotFound)
    }
}
