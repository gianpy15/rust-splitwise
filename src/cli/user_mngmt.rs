use std::error::Error;
use std::fmt::Display;

use inquire::Text;

use crate::db::establish_connection;
use crate::db::queries::user_queries;

use super::base::CliAction;
use super::clear_console;
use crate::context;

#[derive(PartialEq, Eq)]
pub enum UserAction {
    Login,
    Logout,
    Register,
    Back,
}

impl CliAction<UserAction> for UserAction {
    fn perform_action(&self, ctx: &mut context::Context) -> Result<(), Box<dyn Error>> {
        let connection = &mut establish_connection();
        match self {
            UserAction::Login => {
                let username = Text::new("What is your username?").prompt()?;
                match user_queries::get_user(connection, username.as_str()) {
                    Ok(user) => {
                        ctx.set_user(&Some(user));
                        clear_console();
                        println!("Welcome back {}!", username)
                    }
                    Err(_) => println!("Username not found."),
                }
            }
            UserAction::Logout => {
                clear_console();
                ctx.set_user(&None)
            }
            UserAction::Register => {
                let username = Text::new("What is your username?").prompt()?;
                match user_queries::create_user(connection, username.as_str()) {
                    Ok(_) => {
                        let user = user_queries::get_user(connection, username.as_str())?;
                        ctx.set_user(&Some(user));
                        clear_console();
                        println!("Welcome {}!", username);
                    }
                    Err(_) => println!("Username already present."),
                }
            }
            UserAction::Back => (),
        }

        Ok(())
    }

    fn get_options(ctx: &mut context::Context) -> Vec<UserAction> {
        match ctx.get_logged_user() {
            Some(_) => vec![UserAction::Logout, UserAction::Back],
            None => vec![UserAction::Login, UserAction::Register, UserAction::Back],
        }
    }
}

impl Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserAction::Login => write!(f, "Login"),
            UserAction::Logout => write!(f, "Logout"),
            UserAction::Register => write!(f, "Register"),
            UserAction::Back => write!(f, "Back"),
        }
    }
}
