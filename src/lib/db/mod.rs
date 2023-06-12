pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use self::models::{NewUser, Users};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_post(conn: &mut SqliteConnection, username: &str) -> Users {
    use crate::lib::db::schema::users;

    let new_post = NewUser { username };

    diesel::insert_into(users::table)
        .values(&new_post)
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}