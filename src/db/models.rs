pub mod user {
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
}
pub mod group {
    use std::fmt::Display;

    use crate::db::schema::{group_to_user, split_group};
    use diesel::prelude::*;

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = split_group)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Group {
        pub id: i32,
        pub name: String,
    }

    #[derive(Insertable)]
    #[diesel(table_name = split_group)]
    pub struct NewGroup<'a> {
        pub name: &'a str,
    }

    impl Display for Group {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{} ->\t{}", self.id, self.name)?;
            writeln!(f, "-----------\n")?;
            Ok(())
        }
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = group_to_user)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct GroupToUser {
        pub group_id: i32,
        pub user_id: i32,
    }

    #[derive(Insertable)]
    #[diesel(table_name = group_to_user)]
    pub struct NewGroupToUser<'a> {
        pub user_id: &'a i32,
        pub group_id: &'a i32,
    }
}
