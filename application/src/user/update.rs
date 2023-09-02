use diesel::prelude::*;
use domain::models::User;
use infrastructure::database::connection::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn update(user: User) -> Result<User, NotFound<String>> {
    use domain::schema::users::dsl::*;

    match diesel::update(users.find(&user.id))
        .set((
            id.eq(&user.id),
            name.eq(user.name),
            password.eq(user.password),
            email.eq(user.email),
            bio.eq(user.bio),
            profile_picture_url.eq(user.profile_picture_url),
        ))
        .get_result::<User>(&mut establish_connection())
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
