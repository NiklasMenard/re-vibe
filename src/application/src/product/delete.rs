use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::{models::Product, schema::products};
use infrastructure::database::connection::DbPool;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};

pub async fn delete_product(pool: &DbPool, id: i32) -> Result<String, Status> {
    let mut connect = pool.get().await.map_err(|_| Status::InternalServerError)?;

    let num_deleted = match diesel::delete(products::table.filter(products::product_id.eq(id)))
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

    if num_deleted > 0 {
        match products::table
            .load::<Product>(&mut connect)
            .await
        {
            Ok(products_) => {
                let response = Response {
                    body: ResponseBody::Message(format!("Successfully deleted product. {} products remaining.", products_.len())),
                };
                Ok(serde_json::to_string(&response).unwrap())
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
