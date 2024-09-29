use diesel::prelude::*;
use domain::{models::Product, schema::products};
use infrastructure::database::connection::establish_connection;
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};

pub async fn delete_product(id: i32) -> Result<String, Status> {
    let connect = &mut establish_connection();

    let num_deleted = match diesel::delete(products::table.filter(products::product_id.eq(id)))
        .execute(connect)
    {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                return Err(Status::NotFound);
            }
            _ => {
                eprintln!("Database error - {}", err);
                return Err(Status::InternalServerError);
            }
        },
    };

    if num_deleted > 0 {
        match products::table
            .load::<Product>(connect) // Use the existing connection
        {
            Ok(products_) => {
                let response = Response {
                    body: ResponseBody::Message(format!("Successfully deleted product. {} products remaining.", products_.len())),
                };
                Ok(serde_json::to_string(&response).unwrap())
            }
            Err(err) => {
                eprintln!("Database error - {}", err);
                return Err(Status::InternalServerError);
            }
        }
    } else {
        Err(Status::NotFound)
    }
}
