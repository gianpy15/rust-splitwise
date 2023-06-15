use std::error::Error;
use std::fmt::Display;

use inquire::Select;

use inquire::Text;

use crate::db::establish_connection;
use crate::db::queries::group_queries;

use super::base::CliAction;
use super::clear_console;
use crate::context;

#[derive(PartialEq, Eq)]
pub enum GroupAction {
    Create,
    Delete,
    Select,
    AddMember,
    Back,
}

impl CliAction<GroupAction> for GroupAction {
    fn perform_action(&self, ctx: &mut context::Context) -> Result<(), Box<dyn Error>> {
        let connection = &mut establish_connection();
        match self {
            GroupAction::Create => {
                let group_name = Text::new("Choose a name").prompt()?;
                match group_queries::create_group(connection, group_name.as_str()) {
                    Ok(_) => {
                        let group = group_queries::get_group(connection, group_name.as_str())?;
                        ctx.set_group(&Some(group));
                        clear_console();
                        println!("Group {} created!", group_name);
                    }
                    Err(_) => {
                        let _ = Select::new("Group already present.", vec!["Back"]).prompt();
                        return Ok(());
                    }
                }

                group_queries::add_user_to_group(
                    connection,
                    &ctx.get_logged_user().as_ref().unwrap().id,
                    &ctx.get_selected_group().as_ref().unwrap().id,
                )?;
            }
            GroupAction::Delete => {}
            GroupAction::Select => {
                let group = Select::new(
                    "Choose a group:",
                    group_queries::get_user_groups(
                        connection,
                        &ctx.get_logged_user().clone().unwrap(),
                    )?,
                )
                .prompt()?;
                ctx.set_group(&Some(group))
            }
            GroupAction::AddMember => (),
            GroupAction::Back => (),
        }

        Ok(())
    }

    fn get_options(_: &mut context::Context) -> Vec<GroupAction> {
        vec![
            GroupAction::Create,
            GroupAction::Delete,
            GroupAction::Select,
            GroupAction::AddMember,
        ]
    }
}

impl Display for GroupAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GroupAction::Create => write!(f, "Create a new Group"),
            GroupAction::Delete => write!(f, "Delete Group"),
            GroupAction::Select => write!(f, "Select a Group"),
            GroupAction::AddMember => write!(f, "Add member"),
            GroupAction::Back => write!(f, "Back"),
        }
    }
}
