pub mod user_queries {

    use diesel::result::Error;
    use diesel::*;

    use crate::db::models::{NewUser, User};
    use crate::db::schema::user::dsl;

    pub fn load_users(connection: &mut SqliteConnection) -> Result<Vec<User>, Error> {
        dsl::user.select(User::as_select()).load(connection)
    }

    pub fn get_user(connection: &mut SqliteConnection, username: &str) -> Result<User, Error> {
        let users = dsl::user
            .filter(dsl::username.eq(username))
            .select(User::as_select())
            .load(connection)?;
        match users.into_iter().nth(0) {
            Some(user) => Ok(user),
            None => Err(NotFound)
        }
    }

    pub fn create_user(connection: &mut SqliteConnection, username: &str) -> Result<(), Error> {
        use crate::db::schema::user;

        let new_user = NewUser { username };

        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(connection)?;
        Ok(())
    }
}
