use chrono::Utc;
use diesel::RunQueryDsl;
use domain::models::{NewProduct, Product};

use infrastructure::database::connection::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn post_product(product: Json<NewProduct>) -> Created<String> {
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
        creation_date: Utc::now().naive_utc(),
    };

    match diesel::insert_into(products::table)
        .values(&new_product_data)
        .get_result::<Product>(connection)
    {
        Ok(product) => {
            let response = Response {
                body: ResponseBody::Product(product),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
