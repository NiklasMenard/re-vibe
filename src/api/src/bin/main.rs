#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::{Header, Status};
use rocket::{Request, Response};

use api::auth_handler;
use api::catcher_handler;
use api::product_handler;
use api::user_handler;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DB = diesel::pg::Pg;

use infrastructure::database::connection::establish_connection;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Define allowed origins
        let allowed_origins = vec!["http://127.0.0.1:3000", "http://127.0.0.1:8000"];

        if let Some(origin) = request.headers().get_one("Origin") {
            if allowed_origins.contains(&origin) {
                response.set_header(Header::new("Access-Control-Allow-Origin", origin));
                response.set_header(Header::new(
                    "Access-Control-Allow-Methods",
                    "POST, GET, OPTIONS",
                ));
                response.set_header(Header::new(
                    "Access-Control-Allow-Headers",
                    "Content-Type, Authorization",
                ));
                response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
            }
        }

        // Handle OPTIONS requests (preflight requests)
        if request.method() == rocket::http::Method::Options {
            if let Some(origin) = request.headers().get_one("Origin") {
                if allowed_origins.contains(&origin) {
                    response.set_header(Header::new("Access-Control-Allow-Origin", origin));
                    response.set_header(Header::new(
                        "Access-Control-Allow-Methods",
                        "POST, GET, OPTIONS",
                    ));
                    response.set_header(Header::new(
                        "Access-Control-Allow-Headers",
                        "Content-Type, Authorization",
                    ));
                    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
                    response.set_status(Status::Ok);
                } else {
                    response.set_status(Status::Forbidden);
                }
            } else {
                response.set_status(Status::Forbidden);
            }
        }
    }
}

pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

#[launch]
fn rocket() -> _ {
    let connection = &mut establish_connection();

    run_db_migrations(connection);

    rocket::build()
        .attach(CORS)
        .register("/", catchers![catcher_handler::unauthorized])
        .mount("/", routes![catcher_handler::all_options])
        .mount(
            "/api",
            routes![
                product_handler::list_products_handler,
                product_handler::list_product_handler,
                product_handler::create_product_handler,
                product_handler::update_product_handler,
                product_handler::delete_product_handler,
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
        .mount(
            "/auth",
            routes![
                auth_handler::login_handler,
                auth_handler::refresh_token_handler
            ],
        )
        .mount("/", FileServer::from("UI/dist"))
}
