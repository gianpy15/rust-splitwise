pub mod user_queries {
    use diesel::result::Error;
    use diesel::*;

    use crate::lib::db::models::{NewUser, Users};
    use crate::lib::db::schema::users::dsl::users;

    pub fn load_users(connection: &mut SqliteConnection) -> Result<Vec<Users>, Error> {
        users.select(Users::as_select()).load(connection)
    }

    pub fn create_user(connection: &mut SqliteConnection, username: &str) -> Result<(), Error> {
        use crate::lib::db::schema::users;

        let new_user = NewUser { username };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;
        Ok(())
    }
}
