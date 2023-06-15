use std::fmt::Display;

use crate::db::models::{group::Group, user::User};

#[derive(Default)]
pub struct Context {
    logged_user: Option<User>,
    selected_group: Option<Group>,
}

impl Context {
    pub fn get_logged_user(&self) -> &Option<User> {
        &self.logged_user
    }

    pub fn get_selected_group(&self) -> &Option<Group> {
        &self.selected_group
    }

    pub fn set_user(&mut self, user: &Option<User>) {
        self.logged_user = user.clone();
        self.selected_group = None;
    }

    pub fn set_group(&mut self, group: &Option<Group>) {
        self.selected_group = group.clone();
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.logged_user {
            Some(user) => {
                writeln!(f, "--- {} ---", user.username)?;
                writeln!(f, "--- Balance: {} ---", user.balance)?;
            }
            None => (),
        }

        match &self.selected_group {
            Some(group) => writeln!(f, "--- Group: {} ---", group.name)?,
            None => (),
        };

        Ok(())
    }
}
