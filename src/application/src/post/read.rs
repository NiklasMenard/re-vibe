use diesel::prelude::*;
use domain::{models::Post, schema::posts};
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_posts() -> Vec<Post> {
    let connect = &mut establish_connection();

    match posts::table
        .select(posts::all_columns)
        .load::<Post>(connect)
    {
        Ok(mut posts) => {
            posts.sort();
            posts
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_post(post_id: i32) -> Result<Post, NotFound<String>> {
    let connect = &mut establish_connection();

    match posts::table.find(post_id).first::<Post>(connect) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting post with id {} - {}",
                        post_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
