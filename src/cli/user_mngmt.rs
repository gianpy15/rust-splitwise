use std::error::Error;
use std::fmt::Display;

use inquire::{Select, Text};

use crate::db::establish_connection;
use crate::db::queries::user_queries;

use super::base::CliAction;
use crate::context;

#[derive(PartialEq, Eq)]
enum UserAction {
    Login,
    Logout,
    Register,
    Exit
}

impl CliAction for UserAction {
    fn perform_action(&self, ctx: &mut context::Context) -> Result<(), Box<dyn Error>> {
        let connection = &mut establish_connection();
        match self {
            UserAction::Login => {
                let username = Text::new("What is your username?").prompt()?;
                match user_queries::get_user(connection, username.as_str()) {
                    Ok(user) => {
                        ctx.logged_user = Some(user);
                        println!("Welcome back {}!", username)
                    }
                    Err(_) => println!("Username not found."),
                }
            },
            UserAction::Logout => ctx.logged_user = None,
            UserAction::Register => loop {
                let username = Text::new("What is your username?").prompt()?;
                match user_queries::create_user(connection, username.as_str()) {
                    Ok(_) => {
                        let user = user_queries::get_user(connection, username.as_str())?;
                        ctx.logged_user = Some(user);
                        println!("Welcome {}!", username);
                        break;
                    }
                    Err(_) => println!("Username already present.")
                }
            },
            UserAction::Exit => (),
        }

        Ok(())
    }
}

impl Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserAction::Login => write!(f, "Login"),
            UserAction::Logout => write!(f, "Logout"),
            UserAction::Register => write!(f, "Register"),
            UserAction::Exit => write!(f, "Exit"),
        }
    }
}

impl UserAction {
    fn get_options(ctx: &context::Context) -> Vec<UserAction> {
        match ctx.logged_user {
            Some(_) => vec![UserAction::Logout, UserAction::Exit],
            None => vec![UserAction::Login, UserAction::Register, UserAction::Exit],
        }
    }
}

pub fn welcome(ctx: &mut context::Context) -> Result<(), Box<dyn Error>> {
    loop {
        let options = UserAction::get_options(ctx);
        let action = Select::new("What do you want to do?", options).prompt()?;
        action.perform_action(ctx)?;
        if action == UserAction::Exit {
            break;
        }
    }
    Ok(())
}
