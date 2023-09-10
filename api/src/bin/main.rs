#[macro_use]
extern crate rocket;

use api::auth_handler;
use api::post_handler;
use api::user_handler;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DB = diesel::pg::Pg;

use infrastructure::database::connection::establish_connection;

pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

#[launch]
fn rocket() -> _ {
    let connection = &mut establish_connection();

    run_db_migrations(connection);

    rocket::build()
        .mount(
            "/api",
            routes![
                post_handler::list_posts_handler,
                post_handler::list_post_handler,
                post_handler::create_post_handler,
                post_handler::update_post_handler,
                post_handler::delete_post_handler,
            ],
        )
        .mount(
            "/user",
            routes![
                user_handler::register_user_handler,
                user_handler::list_user_handler,
                user_handler::update_user_handler,
                user_handler::delete_user_handler
            ],
        )
        .mount("/auth", routes![auth_handler::login_handler,])
}
