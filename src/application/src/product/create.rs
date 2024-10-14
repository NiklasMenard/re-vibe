use chrono::Utc;
use diesel::{ExpressionMethods, RunQueryDsl};
use domain::models::{NewProduct, Product};

use infrastructure::database::connection::establish_connection;
use rocket::http::Status;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub async fn post_product(product: Json<NewProduct>) -> Result<String, Status> {
    use domain::schema::products;

    let new_data = product.into_inner();
    let connection = &mut establish_connection();

    let new_product_data = NewProduct {
        name: new_data.name,
        description: new_data.description,
        price: new_data.price,
        quantity: new_data.quantity,
        seller_id: new_data.seller_id,
        category_id: new_data.category_id,
        bucket_key: new_data.bucket_key,
    };

    match diesel::insert_into(products::table)
        .values((
            &new_product_data,
            products::creation_date.eq(Utc::now().naive_utc()),
        ))
        .get_result::<Product>(connection)
    {
        Ok(_) => {
            let response = Response {
                body: ResponseBody::Message("Product created successfully".to_string()),
            };
            Ok(serde_json::to_string(&response).unwrap())
        }
        Err(err) => {
            eprintln!("Database error - {}", err);
            Err(Status::InternalServerError)
        }
    }
}
