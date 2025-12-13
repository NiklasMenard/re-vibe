use diesel::{dsl::count_star, prelude::*};
use diesel_async::RunQueryDsl;
use domain::{
    models::{PaginatedProducts, Product},
    schema::products,
};
use infrastructure::{
    database::connection::DbPool,
    s3_client::{create_client, generate_presigned_url},
};
use rocket::{http::Status, serde::json::Json};
use shared::{
    request_models::ProductFilter,
    response_models::{Response, ResponseBody},
};

pub async fn list_products(
    pool: &DbPool,
    page: Option<i64>,
    page_size: Option<i64>,
    filter: Option<Json<ProductFilter>>,
) -> Result<String, Status> {
    let mut connect = pool.get().await.map_err(|_| Status::InternalServerError)?;

    let mut query = products::table.select(products::all_columns).into_boxed();

    if let Some(filter_json) = filter {
        query = query.filter(products::name.like(format!("%{}%", filter_json.into_inner().name)));
    }

    let total_count: i64 = match products::table.select(count_star()).first(&mut connect).await {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Database error - {}", err);
            return Err(Status::InternalServerError);
        }
    };

    let page = page.unwrap_or(1).max(1);
    let page_size = page_size.unwrap_or(5).max(5);

    let offset: i64 = (page - 1) * page_size.min(total_count);

    // Fetch products with pagination
    let products = query
        .limit(page_size)
        .offset(offset)
        .load::<Product>(&mut connect)
        .await
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
        .await;

        match image_url {
            Ok(url) => {
                let product = Product {
                    product_id: product.product_id,
                    name: product.name,
                    description: product.description,
                    price: product.price,
                    quantity: product.quantity,
                    seller_id: product.seller_id,
                    category_id: product.category_id,
                    creation_date: product.creation_date,
                    bucket_key: url,
                };

                products_with_urls.push(product);
            }
            Err(err) => {
                eprintln!("S3 error - {}", err);
                return Err(Status::InternalServerError);
            }
        }
    }

    products_with_urls.sort_by(|a, b| a.name.cmp(&b.name));

    let response = Response {
        body: ResponseBody::PaginatedProducts(PaginatedProducts {
            products: products_with_urls,
            total_count,
            total_pages: (total_count + page_size - 1) / page_size,
            current_page: page,
        }),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

pub async fn list_product(pool: &DbPool, product_id: i32) -> Result<String, Status> {
    let mut connect = pool.get().await.map_err(|_| Status::InternalServerError)?;

    let product = match products::table.find(product_id).first::<Product>(&mut connect).await {
        Ok(product) => {
            let client = create_client().await.unwrap();

            let bucket_key_with_extension = format!("large_images/{}.jpg", product.bucket_key);

            let image_url = generate_presigned_url(
                &client,
                "re-vibe",
                &bucket_key_with_extension,
                3600, // URL expiration time in seconds
            )
            .await;

            match image_url {
                Ok(url) => {
                    let product = Product {
                        product_id: product.product_id,
                        name: product.name,
                        description: product.description,
                        price: product.price,
                        quantity: product.quantity,
                        seller_id: product.seller_id,
                        category_id: product.category_id,
                        creation_date: product.creation_date,
                        bucket_key: url,
                    };

                    let response = Response {
                        body: ResponseBody::Product(product),
                    };

                    Ok(serde_json::to_string(&response).unwrap())
                }
                Err(err) => {
                    eprintln!("S3 error - {}", err);
                    Err(Status::InternalServerError)
                }
            }
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => Err(Status::NotFound),
            _ => {
                eprintln!("Database error - {}", err);
                Err(Status::InternalServerError)
            }
        },
    };

    product
}
