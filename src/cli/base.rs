use std::error::Error;
use std::fmt::Display;

use inquire::Select;
use super::user_mngmt::UserAction;
use super::group_mngmt::GroupAction;

use crate::context::Context;

pub trait CliAction<T> {
    fn perform_action(&self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
    fn get_options(ctx: &mut Context) -> Vec<T>;
}

pub fn welcome(ctx: &mut Context) -> Result<(), Box<dyn Error>> {
    println!("Welcome to Rust Splitwise!");
    loop {
        let options = match ctx.logged_user {
            Some(_) => vec!["User", "Groups", "Exit"],
            None => vec!["User", "Exit"]
        };
        match Select::new("Select group:", options).prompt()? {
            "User" => {
                let action = Select::new("What do you want to do?", UserAction::get_options(ctx)).prompt()?;
                action.perform_action(ctx)?;
            },
            "Groups" => {
                let action = Select::new("What do you want to do?", GroupAction::get_options(ctx)).prompt()?;
                action.perform_action(ctx)?;
            },
            _ => break
        }
    }
    Ok(())
}