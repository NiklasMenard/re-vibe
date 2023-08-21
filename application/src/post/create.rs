use chrono::Utc;
use diesel::RunQueryDsl;
use domain::models::{NewPost, Post};
use infrastructure::database::establish_connection;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let new_data = post.into_inner();
    let connection = &mut establish_connection();

    let new_post = NewPost {
        title: new_data.title,
        content: new_data.content,
        publication_date: Some(Utc::now().naive_utc()),
        author_id: new_data.author_id,
        category_id: new_data.category_id,
    };

    match diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(connection)
    {
        Ok(post) => {
            let response = Response {
                body: ResponseBody::Post(post),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
