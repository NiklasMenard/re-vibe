use diesel::prelude::*;
use domain::{models::Product, schema::products};
use infrastructure::{
    database::connection::establish_connection,
    s3_client::{create_client, generate_presigned_url},
};
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};

pub async fn list_products() -> Result<String, Status> {
    let connect = &mut establish_connection();

    let products = products::table
        .select(products::all_columns)
        .load::<Product>(connect)
        .map_err(|err| {
            eprintln!("Database error - {}", err);
            Status::InternalServerError
        })?;

    //Create S3 Client
    let client = create_client().await.unwrap();

    // Fetch S3 client
    let mut products_with_urls = Vec::new();
    for product in products {
        let bucket_key_with_extension = format!("medium_images/{}.jpg", &product.bucket_key);

        let image_url = generate_presigned_url(
            &client,
            "re-vibe",
            &bucket_key_with_extension,
            3600, // URL expiration time in seconds
        )
        .await
        .unwrap();

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

    products_with_urls.sort_by(|a, b| a.name.cmp(&b.name));

    let response = Response {
        body: ResponseBody::Products(products_with_urls),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

pub async fn list_product(product_id: i32) -> Result<String, Status> {
    let connect = &mut establish_connection();

    match products::table.find(product_id).first::<Product>(connect) {
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
