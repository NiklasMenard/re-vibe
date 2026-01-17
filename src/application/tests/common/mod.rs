use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use infrastructure::database::connection::DbPool;
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

/// Clean up test user by email
pub async fn cleanup_test_user(pool: &DbPool, email: &str) {
    use domain::schema::{user_roles, users};

    let mut conn = pool.get().await.unwrap();

    // First, get the user ID to delete associated user_roles
    if let Ok(user) = users::table
        .filter(users::email.eq(email))
        .select(users::id)
        .first::<uuid::Uuid>(&mut conn)
        .await
    {
        // Delete user_roles first (foreign key constraint)
        diesel::delete(user_roles::table.filter(user_roles::user_id.eq(user)))
            .execute(&mut conn)
            .await
            .ok();
    }

    // Then delete the user
    diesel::delete(users::table.filter(users::email.eq(email)))
        .execute(&mut conn)
        .await
        .ok();
}

/// Create a test user
pub async fn create_test_user(pool: &DbPool, email: &str, password: &str) {
    use domain::models::{NewUser, NewUserRole};
    use domain::schema::{user_roles, users};

    let mut conn = pool.get().await.unwrap();

    let new_user = NewUser::from_credentials(email, password);

    // Insert user and get the created user to access the ID
    let user: domain::models::User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
        .await
        .expect("Failed to create test user");

    // Create user role
    let new_user_role = NewUserRole {
        user_id: user.id,
        role_id: 1,
    };

    diesel::insert_into(user_roles::table)
        .values(&new_user_role)
        .execute(&mut conn)
        .await
        .ok();
}
