pub mod lib;

use crate::lib::db::queries::user_queries;

fn main() {
    let connection = &mut lib::db::establish_connection();
    let username = "Gioo";
    match user_queries::create_user(connection, username) {
        Ok(_) => println!("Welcome, {username}!"),
        Err(error) => println!("Cannot insert user {username}, {error:?}"),
    }
    match user_queries::load_users(connection) {
        Ok(all_users) => {
            println!("Found {} users.", all_users.len());
            for user in all_users {
                println!("{:?} -> {}", user.id, user.username);
                println!("-----------\n");
            }
        }
        Err(_) => (),
    }
}
