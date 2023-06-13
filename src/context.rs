use crate::db::models::User;

#[derive(Default)]
pub struct Context {
    pub logged_user: Option<User>,
}
