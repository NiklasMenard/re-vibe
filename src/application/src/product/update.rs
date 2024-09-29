use diesel::prelude::*;
use domain::models::{NewProduct, Product};
use infrastructure::database::connection::establish_connection;
use rocket::{http::Status, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

pub async fn update_product(id: i32, new_product: Json<NewProduct>) -> Result<String, Status> {
    use domain::schema::products::dsl::*;

    let product_update = new_product.into_inner();

    match diesel::update(products.find(id))
        .set((
            name.eq(product_update.name),
            description.eq(product_update.description),
            price.eq(product_update.price),
            quantity.eq(product_update.quantity),
            category_id.eq(product_update.category_id),
            bucket_key.eq(product_update.bucket_key),
        ))
        .get_result::<Product>(&mut establish_connection())
    {
        Ok(product) => {
            let response = Response {
                body: ResponseBody::Product(product),
            };

            Ok(serde_json::to_string(&response).unwrap())
        }

        Err(err) => match err {
            diesel::result::Error::NotFound => Err(Status::NotFound),
            _ => {
                eprintln!("Database error - {}", err);
                Err(Status::InternalServerError)
            }
        },
    }
}
