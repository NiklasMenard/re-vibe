use argon2::Config;
use diesel::prelude::*;
use domain::{
    models::{InsertableUser, User},
    schema::users,
};
use infrastructure::database::connection::establish_connection;
use uuid::Uuid;

pub fn register_user(user: User) -> QueryResult<User> {
    let connection = &mut establish_connection();

    let new_user = User {
        id: Uuid::new_v4(),
        email: user.email.to_string(),
        password: user.password.to_string(),
        name: user.name.to_string(),
        bio: user.bio,
        profile_picture_url: user.profile_picture_url,
    };

    diesel::insert_into(users::table)
        .values(&InsertableUser::from_user(new_user))
        .get_result::<User>(connection)
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
