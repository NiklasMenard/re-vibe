// infrastructure/src/lib.rs

pub mod connection {
    use diesel_async::pooled_connection::deadpool::Pool as AsyncPool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::AsyncPgConnection;
    use dotenvy::dotenv;
    use std::env;

    pub type DbPool = AsyncPool<AsyncPgConnection>;

    pub fn get_pool() -> DbPool {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        AsyncPool::builder(config)
            .build()
            .expect("Failed to create pool")
    }
}
