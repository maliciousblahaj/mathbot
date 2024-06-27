use std::future::Future;

use crate::Result;
use serenity::all::{Context, Message};

/// The category a root command can have.
/// 
/// Having the category of Admin means the command only generates in the help menu for admins
/// and having the category of Test means the command does not show up in the help menu at all
pub enum CommandCategory {
    Info,
    User,
    Currency,
    Fun,
    Math,
    Admin,
    Test,
}

/// A command can either be a root command or a subcommand.
/// 
/// Root commands have a category assigned to them, but subcommands cannot.
pub enum CommandType {
    RootCommand{category: CommandCategory},
    SubCommand{parent: Box<Command>}
}

pub struct Command
    //F: Future<Output = Result<()>>,
{
    handle: Box<fn(CommandParams) -> dyn Future<Output = Result<()>>>,
    aliases: Vec<String>,
    cmd_type: CommandType,

    //TODO: add documentation for commands (for help menu)
}

impl Command {
    pub fn new(
        handle: fn(CommandParams) -> dyn Future<Output = Result<()>>,
        aliases: Vec<String>,
        cmd_type: CommandType,
    ) -> Self 
    {
        Self {
            handle: Box::new(handle),
            aliases,
            cmd_type,
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