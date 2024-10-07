use diesel::prelude::*;
use domain::{
    models::Product,
    schema::{
        products, user_favorite_products,
        users::{self},
    },
};
use infrastructure::{
    database::connection::establish_connection,
    s3_client::{create_client, generate_presigned_url},
};
use rocket::http::Status;
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn list_favorite_products(user_id: Uuid) -> Result<String, Status> {
    let connect = &mut establish_connection();

    let products = user_favorite_products::table
        .inner_join(users::table.on(user_favorite_products::user_id.eq(users::id)))
        .inner_join(products::table.on(user_favorite_products::product_id.eq(products::product_id)))
        .filter(user_favorite_products::user_id.eq(user_id))
        .select((
            products::product_id,
            products::name,
            products::description,
            products::price,
            products::quantity,
            products::seller_id,
            products::category_id,
            products::creation_date,
            products::bucket_key,
        ))
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
