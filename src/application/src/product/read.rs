use diesel::prelude::*;
use domain::{models::Product, schema::products};
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_products() -> Vec<Product> {
    let connect = &mut establish_connection();

    match products::table
        .select(products::all_columns)
        .load::<Product>(connect)
    {
        Ok(mut posts) => {
            posts.sort();
            posts
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_product(post_id: i32) -> Result<Product, NotFound<String>> {
    let connect = &mut establish_connection();

    match products::table.find(post_id).first::<Product>(connect) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting post with id {} - {}",
                        post_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
