use diesel::prelude::*;
use domain::models::Product;
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_product(id: i32) -> Result<Vec<Product>, NotFound<String>> {
    use domain::schema::products;
    use domain::schema::products::dsl::*;

    let response: Response;
    let connect = &mut establish_connection();

    let num_deleted = match diesel::delete(products.filter(product_id.eq(id))).execute(connect) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error publishing post with id {:?} - {:?}",
                        product_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if num_deleted > 0 {
        match products::table
            .select(products::all_columns)
            .load::<Product>(&mut establish_connection())
        {
            Ok(mut products_) => {
                products_.sort();
                Ok(products_)
            }
            // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no post with id {:?}", product_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
