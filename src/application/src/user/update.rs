use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::models::User;
use infrastructure::database::connection::DbPool;
use rocket::{response::status::NotFound, serde::json::Json};
use shared::response_models::{Response, ResponseBody};
use uuid::Uuid;

pub async fn update_user(pool: &DbPool, user_id: String, user: Json<User>) -> Result<User, NotFound<String>> {
    use domain::schema::users::dsl::*;

    let post_to_update = user.into_inner();
    let mut connection = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            let response = Response {
                body: ResponseBody::Message("Database connection error".to_string()),
            };
            return Err(NotFound(serde_json::to_string(&response).unwrap()));
        }
    };

    match diesel::update(users.find(Uuid::parse_str(&user_id).unwrap()))
        .set((
            id.eq(&post_to_update.id),
            name.eq(post_to_update.name),
            password.eq(post_to_update.password),
            email.eq(post_to_update.email),
            bio.eq(post_to_update.bio),
            profile_picture_url.eq(post_to_update.profile_picture_url),
        ))
        .get_result::<User>(&mut connection)
        .await
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error updating user with id {:?} - {:?}",
                        id, err
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
