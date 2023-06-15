pub mod user_queries {

    use diesel::result::Error;
    use diesel::*;

    use crate::db::models::user::{NewUser, User};
    use crate::db::schema::user::dsl;
    use crate::db::schema::user;

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
            None => Err(NotFound),
        }
    }

    pub fn create_user(connection: &mut SqliteConnection, username: &str) -> Result<(), Error> {

        let new_user = NewUser { username };

        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(connection)?;
        Ok(())
    }
}

pub mod group_queries {
    use diesel::result::Error;
    use diesel::*;
    

    use crate::db::models::group::{NewGroup, NewGroupToUser, Group};
    use crate::db::schema::split_group::dsl as gdsl;
    use crate::db::schema::group_to_user::dsl as gudsl;
    use crate::db::schema::{split_group, group_to_user};

    pub fn create_group(connection: &mut SqliteConnection, name: &str) -> Result<(), Error> {
        let new_group = NewGroup {name};
        diesel::insert_into(split_group::table).values(&new_group).execute(connection)?;
        Ok(())
    }

    pub fn add_user_to_group(connection: &mut SqliteConnection, user_id: &i32, group_id: &i32) -> Result<(), Error> {
        let new_connection = NewGroupToUser {user_id, group_id};
        diesel::insert_into(group_to_user::table).values(new_connection).execute(connection)?;
        Ok(())
    }

    pub fn get_group(connection: &mut SqliteConnection, name: &str) -> Result<Group, Error> {
        let groups = gdsl::split_group
            .filter(gdsl::name.eq(name))
            .select(Group::as_select())
            .load(connection)?;
        match groups.into_iter().nth(0) {
            Some(user) => Ok(user),
            None => Err(NotFound),
        }
    }
}