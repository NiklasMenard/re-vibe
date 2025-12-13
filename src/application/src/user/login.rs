use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::{
    models::{Role, User, UserRole},
    schema::{roles, user_roles, users},
};
use infrastructure::database::connection::DbPool;

pub async fn check_email_password(pool: &DbPool, email: String, password: String) -> Option<UserRole> {
    let mut connection = pool.get().await.ok()?;

    // Use manual join conditions to get user info and associated role
    match users::table
        .filter(users::email.eq(&email))
        .inner_join(
            user_roles::table
                .on(users::id.eq(user_roles::user_id))
                .inner_join(roles::table.on(user_roles::role_id.eq(roles::role_id))),
        )
        .select((users::all_columns, roles::all_columns))
        .first::<(User, Role)>(&mut connection)
        .await
    {
        Ok((user, role)) => {
            let hash = User::hash_with_salt(&password, &user.salt);

            if user.password == hash {
                let new_user_role = UserRole {
                    user_id: user.id,
                    role_id: role.role_id,
                };

                return Some(new_user_role);
            } else {
                return None;
            }
        }

        Err(_) => None,
    }
}
