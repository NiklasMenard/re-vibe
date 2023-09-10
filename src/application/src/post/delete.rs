use diesel::prelude::*;
use domain::models::Post;
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_post(id: i32) -> Result<Vec<Post>, NotFound<String>> {
    use domain::schema::posts;
    use domain::schema::posts::dsl::*;

    let response: Response;
    let connect = &mut establish_connection();

    let num_deleted = match diesel::delete(posts.filter(post_id.eq(id))).execute(connect) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error publishing post with id {:?} - {:?}",
                        post_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if num_deleted > 0 {
        match posts::table
            .select(posts::all_columns)
            .load::<Post>(&mut establish_connection())
        {
            Ok(mut posts_) => {
                posts_.sort();
                Ok(posts_)
            }
            // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no post with id {:?}", post_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
