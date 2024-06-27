use std::future::Future;

use crate::Result;
use serenity::all::{Context, Message};

pub enum CommandGroup {
    Info,
    User,
    Currency,
    Fun,
    Math,
    Admin,
    Test,
}

pub struct Command
    //F: Future<Output = Result<()>>,
{
    handle: Box<fn(CommandParams) -> dyn Future<Output = Result<()>>>,
    aliases: Vec<String>,
    group: CommandGroup,

    //TODO: add documentation for commands (for help menu)
}

impl Command {
    pub fn new(
        handle: fn(CommandParams) -> dyn Future<Output = Result<()>>,
        aliases: Vec<String>,
        group: CommandGroup,
    ) -> Self 
    {
        Self {
            handle: Box::new(handle),
            aliases,
            group,
        }
    }
}


/// Struct for parameters and context to a command
/// 
/// will include things like args in the future
pub struct CommandParams {
    ctx: Context, 
    msg: Message,

    // TODO: add more things like args
}