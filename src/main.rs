use std::process::exit;

use splitwise::db::establish_connection;
use splitwise::db::queries::user_queries;
use splitwise::{cli, context};
fn main() {
    app();
}

fn app() {
    let connection = &mut establish_connection();
    let mut context: context::Context = Default::default();
    if !cli::base::welcome(&mut context).is_ok() {
        exit(-1);
    }

    match user_queries::load_users(connection) {
        Ok(all_users) => {
            println!("Found {} users.", all_users.len());
            for user in all_users {
                println!("{user}");
            }
        }
        Err(_) => (),
    }
}
