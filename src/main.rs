pub mod lib;

use diesel::prelude::*;

use crate::lib::db::models::Users;

fn main() {
    use self::lib::db::schema::users::dsl::*;

    let connection = &mut lib::db::establish_connection();
    let results = users
        .limit(5)
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.username);
        println!("{:?}", user.id);
        println!("-----------\n");
    }
}
