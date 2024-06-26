use crate::{Error, Result};
use serenity::all::{Context, Message};

pub enum CommandGroup {
    Info,
    User,
    Currency,
    Fun,
    Math,
    Admin,
}

pub struct Command<F> 
    where F: FnMut(CommandParams) -> Result<()>
{
    handle: F,
    aliases: Vec<String>,
    group: CommandGroup,

    //TODO: add documentation for commands (for help menu)
}

//constructor boilerplate
impl Command {
    pub fn new(
        handle: fn(CommandParams) -> Result<()>,
        aliases: Vec<String>,
        group: CommandGroup,
    ) -> Self {
        Self {
            handle,
            aliases,
            group,
        }
    }
}


/// Struct for parameters to a command
/// 
/// Includes Context and Message right now, but will include things like args in the future
pub struct CommandParams {
    ctx: Context, 
    msg: Message,

    // TODO: add more things like args
}