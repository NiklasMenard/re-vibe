use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use infrastructure::database::connection::DbPool;
use rocket::local::asynchronous::Client;
use std::env;

/// Setup test database connection pool
pub async fn setup_test_pool() -> DbPool {
    // Load .env.test explicitly for tests
    dotenvy::from_filename(".env.test").ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:testpassword@localhost:5434/re_vibe_test".to_string()
    });

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .build()
        .expect("Failed to create test pool")
}

/// Setup Rocket test client with test database
pub async fn setup_test_client() -> Client {
    use api::{auth_handler, user_handler};

    let pool = setup_test_pool().await;

    let rocket = rocket::build()
        .manage(pool)
        .mount(
            "/api/auth",
            rocket::routes![
                auth_handler::login_handler,
                auth_handler::refresh_token_handler,
                auth_handler::logout,
            ],
        )
        .mount(
            "/api/users",
            rocket::routes![user_handler::register_user_handler,],
        );

    Client::tracked(rocket)
        .await
        .expect("Failed to create test client")
}
