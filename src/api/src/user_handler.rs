use domain::models::User;
use infrastructure::auth::AdminApiKey;
use infrastructure::database::connection::DbPool;

use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};

use shared::request_models::Credentials;
use shared::response_models::{Response, ResponseBody};

use uuid::Uuid;

use application::user::{create, delete, read, update};

#[get("/<id>")]
pub async fn list_user_handler(pool: &State<DbPool>, _key: AdminApiKey, id: String) -> Result<String, Status> {
    let user = read::get_by_id(pool.inner(), Uuid::parse_str(&id).unwrap()).await.map_err(|_| Status::NotFound)?;

    let response = Response {
        body: ResponseBody::User(user),
    };
    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<id>", data = "<user>")]
pub async fn update_user_handler(
    pool: &State<DbPool>,
    _key: AdminApiKey,
    id: String,
    user: Json<User>,
) -> Result<String, Status> {
    let user = update::update_user(pool.inner(), id, user).await.map_err(|_| Status::NotFound)?;

    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[delete("/<id>")]
pub async fn delete_user_handler(pool: &State<DbPool>, _key: AdminApiKey, id: String) -> Status {
    delete::delete_user(pool.inner(), Uuid::parse_str(&id).unwrap()).await.ok();
    Status::NoContent
}

#[post("/register", format = "application/json", data = "<credentials>")]
pub async fn register_user_handler(pool: &State<DbPool>, credentials: Json<Credentials>) -> Status {
    create::register_user(pool.inner(), credentials).await
}
