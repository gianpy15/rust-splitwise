use crate::db::models::{user::User, group::Group};

#[derive(Default)]
pub struct Context {
    pub logged_user: Option<User>,
    pub selected_group: Option<Group>,
}
