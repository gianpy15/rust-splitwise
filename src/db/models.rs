use std::fmt::Display;

use crate::db::schema::user;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub balance: f64,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub username: &'a str,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ->\t{}", self.id, self.username)?;
        writeln!(f, "\tBalance: {}", self.balance)?;
        writeln!(f, "-----------\n")?;
        Ok(())
    }
}
