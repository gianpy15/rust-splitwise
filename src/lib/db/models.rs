use crate::lib::db::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::lib::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub id: Option<i32>,
    pub username: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
}