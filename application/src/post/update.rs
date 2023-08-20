use diesel::prelude::*;
use domain::models::{NewPost, Post};
use infrastructure::establish_connection;
use rocket::{response::status::NotFound, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

pub fn update_post(id: i32, new_post: Json<NewPost>) -> Result<Post, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let updated_post = new_post.into_inner();

    match diesel::update(posts.find(id))
        .set((
            title.eq(updated_post.title),
            content.eq(updated_post.content),
        ))
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error updating post with id {:?} - {:?}",
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
