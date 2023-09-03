use argon2::Config;
use diesel::prelude::*;
use domain::{
    models::{InsertableUser, User},
    schema::users,
};
use infrastructure::database::connection::establish_connection;
use rocket::{response::status::Created, serde::json::Json};
use shared::{
    request_models::Credentials,
    response_models::{Response, ResponseBody},
};
use uuid::Uuid;

pub fn register_user(credentials: Json<Credentials>) -> Created<String> {
    let connection = &mut establish_connection();

    let new_credentials = credentials.into_inner();

    let new_user = User {
        id: Uuid::new_v4(),
        email: new_credentials.email,
        password: new_credentials.password,
        name: "".to_string(),
        bio: Some("".to_string()),
        profile_picture_url: Some("".to_string()),
    };

    match diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(new_user))
        .get_result::<User>(connection)
    {
        Ok(user) => {
            let response = Response {
                body: ResponseBody::User(user),
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

pub fn register_user_by_email_and_password(email: String, password_: String) -> Option<User> {
    let connection = &mut establish_connection();

    let salt = b"somesalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(&password_.as_bytes(), salt, &config).unwrap();
    println!("Hashed password {:?}", &hash);
    let res = users::table
        .filter(users::email.eq(email))
        .filter(users::password.eq(hash))
        .order(users::id)
        .first(connection);
    match res {
        Ok(user) => Some(user),
        Err(_) => None,
    }
}
