use diesel::prelude::*;
use diesel::QueryDsl;
use domain::schema::users;
use infrastructure::database::connection::establish_connection;
use uuid::Uuid;

pub fn delete(id: Uuid) -> bool {
    let connection = &mut establish_connection();

    diesel::delete(users::table.find(id))
        .execute(connection)
        .is_ok()
}
