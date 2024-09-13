use diesel::prelude::*;
use domain::{models::Product, schema::products};
use infrastructure::{
    database::connection::establish_connection,
    s3_client::{create_client, generate_presigned_url},
};
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub async fn list_products() -> Vec<Product> {
    let connect = &mut establish_connection();

    // Fetch products from the database
    let products = match products::table
        .select(products::all_columns)
        .load::<Product>(connect)
    {
        Ok(products) => products,
        Err(err) => {
            panic!("Database error - {}", err);
        }
    };

    //Create S3 Client
    let client = create_client().await.unwrap();

    // Fetch S3 client
    let mut products_with_urls = Vec::new();
    for product in products {
        let image_url = generate_presigned_url(
            &client,
            "re-vibe",
            &product.bucket_key,
            3600, // URL expiration time in seconds
        )
        .await
        .unwrap(); // Handle the error as needed

        products_with_urls.push(Product {
            product_id: product.product_id,
            name: product.name,
            description: product.description,
            price: product.price,
            quantity: product.quantity,
            seller_id: product.seller_id,
            category_id: product.category_id,
            creation_date: product.creation_date,
            bucket_key: image_url,
        });
    }

    // Sort products if necessary
    products_with_urls.sort_by(|a, b| a.name.cmp(&b.name));

    products_with_urls
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
