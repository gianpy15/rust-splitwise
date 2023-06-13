use std::error::Error;

use crate::context::Context;

pub trait CliAction {
    fn perform_action(&self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
}
