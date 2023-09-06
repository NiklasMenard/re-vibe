use application::post::{create, delete, read, update};
use domain::models::NewPost;
use infrastructure::auth::ApiKey;
use rocket::response::status::{Created, NotFound};

use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::response_models::{Response, ResponseBody};

#[get("/posts")]
pub fn list_posts_handler(_key: ApiKey) -> String {
    let posts = read::list_posts();

    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/posts/<post_id>")]
pub fn list_post_handler(_key: ApiKey, post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;

    let response = Response {
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/posts/<post_id>", format = "application/json", data = "<post>")]
pub fn update_post_handler(
    _key: ApiKey,
    post_id: i32,
    post: Json<NewPost>,
) -> Result<String, NotFound<String>> {
    let post = update::update_post(post_id, post)?;

    let response = Response {
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/posts", format = "application/json", data = "<post>")]
pub fn create_post_handler(_key: ApiKey, post: Json<NewPost>) -> Created<String> {
    create::create_post(post)
}

#[delete("/posts/<id>")]
pub fn delete_post_handler(_key: ApiKey, id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(id)?;

    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
